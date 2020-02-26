// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use autopilot::geometry::{Point, Rect, Size};
use image;
use image::Pixel;
use image::{ImageOutputFormat, ImageResult, Rgba};
use internal::{rgb_to_hex, hex_to_rgb, FromImageError};
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::PyNativeType;
use pyo3::PyObjectProtocol;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[pyclass]
struct Bitmap {
    bitmap: autopilot::bitmap::Bitmap,
}

#[pyproto]
impl PyObjectProtocol for Bitmap {
    fn __richcmp__(&self, other: &Bitmap, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.bitmap == other.bitmap),
            _ => unimplemented!(),
        }
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.bitmap.hash(&mut s);
        Ok(s.finish() as isize)
    }
}

#[pyproto]
impl pyo3::class::PyBufferProtocol for Bitmap {
    // From
    // https://github.com/PyO3/pyo3/blob/97189a1/tests/test_buffer_protocol.rs#L17
    fn bf_getbuffer(&self, view: *mut pyo3::ffi::Py_buffer, flags: libc::c_int) -> PyResult<()> {
        use pyo3::exceptions::BufferError;
        use pyo3::*;
        use std::ffi::CStr;
        use std::ptr;

        if view.is_null() {
            return Err(BufferError::py_err("View is null"));
        }

        unsafe {
            (*view).obj = ptr::null_mut();
        }

        if (flags & ffi::PyBUF_WRITABLE) == ffi::PyBUF_WRITABLE {
            return Err(BufferError::py_err("Object is not writable"));
        }

        let bytes = &self.bitmap.image.raw_pixels();

        unsafe {
            (*view).buf = bytes.as_ptr() as *mut libc::c_void;
            (*view).len = bytes.len() as isize;
            (*view).readonly = 1;
            (*view).itemsize = 1;

            (*view).format = ptr::null_mut();
            if (flags & ffi::PyBUF_FORMAT) == ffi::PyBUF_FORMAT {
                let msg = CStr::from_bytes_with_nul(b"B\0").unwrap();
                (*view).format = msg.as_ptr() as *mut _;
            }

            (*view).ndim = 1;
            (*view).shape = ptr::null_mut();
            if (flags & ffi::PyBUF_ND) == ffi::PyBUF_ND {
                (*view).shape = (&((*view).len)) as *const _ as *mut _;
            }

            (*view).strides = ptr::null_mut();
            if (flags & ffi::PyBUF_STRIDES) == ffi::PyBUF_STRIDES {
                (*view).strides = &((*view).itemsize) as *const _ as *mut _;
            }

            (*view).suboffsets = ptr::null_mut();
            (*view).internal = ptr::null_mut();
        }

        Ok(())
    }
}

#[pymethods]
impl Bitmap {
    /// Saves image to absolute path in the given format. The image type is
    /// determined from the filename if possible, unless format is given. If
    /// the file already exists, it will be overwritten. Currently only jpeg
    /// and png files are supported.
    ///
    /// Exceptions:
    ///     - `IOError` is thrown if the file could not be saved.
    ///     - `ValueError` is thrown if image couldn't be parsed.
    fn save(&self, path: &str, format: Option<&str>) -> PyResult<()> {
        let format = format
            .or(Path::new(path).extension().and_then(|x| x.to_str()))
            .unwrap_or("");
        let fmt = image_output_format_from_extension(format);
        match fmt {
            AutoPyImageFormat::Unsupported => Err(pyo3::exceptions::ValueError::py_err(format!(
                "Unknown format {}",
                format
            ))),
            _ => {
                let ref mut buffer = File::create(path)?;
                self.bitmap
                    .image
                    .write_to(buffer, fmt)
                    .map_err(FromImageError::from)?;
                Ok(())
            }
        }
    }

    /// Copies image to pasteboard. Currently only supported on macOS.
    ///
    /// Exceptions:
    ///     - `IOError` is thrown if the image could not be copied.
    ///     - `ValueError` is thrown if the image was too large or small.
    fn copy_to_pasteboard(&self) -> PyResult<()> {
        self.bitmap
            .copy_to_pasteboard()
            .map_err(FromImageError::from)?;
        Ok(())
    }

    /// Returns `True` if the given point is contained in `bmp.bounds`.
    fn point_in_bounds(&self, x: f64, y: f64) -> PyResult<bool> {
        Ok(self.bitmap.bounds().is_point_visible(Point::new(x, y)))
    }

    /// Returns `True` if the given rect of the form `((x, y), (width,
    /// height))` is contained in `bmp.bounds`.
    fn rect_in_bounds(&self, rect: ((f64, f64), (f64, f64))) -> PyResult<bool> {
        let rect = Rect::new(
            Point::new((rect.0).0, (rect.0).1),
            Size::new((rect.1).0, (rect.1).1),
        );
        Ok(self.bitmap.bounds().is_rect_visible(rect))
    }

    /// Open the image located at the path specified. The image's format is
    /// determined from the path's file extension.
    #[classmethod]
    fn open(cls: &PyType, path: String) -> PyResult<Py<Bitmap>> {
        let image = image::open(path).map_err(FromImageError::from)?;
        let bmp = autopilot::bitmap::Bitmap::new(image, None);
        let result = Py::new(cls.py(), Bitmap { bitmap: bmp })?;
        Ok(result)
    }

    /// Returns hexadecimal value describing the color at a given point.
    ///
    /// Exceptions:
    ///     - `ValueError` is thrown if the point out of bounds.
    fn get_color(&self, x: f64, y: f64) -> PyResult<u32> {
        let point = Point::new(x, y);
        if !self.bitmap.bounds().is_point_visible(point) {
            Err(pyo3::exceptions::ValueError::py_err(format!(
                "Point out of bounds {}",
                point
            )))
        } else {
            let rgb = self.bitmap.get_pixel(point);
            let (r, g, b, _) = rgb.channels4();
            Ok(rgb_to_hex(r, g, b))
        }
    }

    /// Attempts to find `color` inside `rect` of the form `((x, y), (width,
    /// height))` in `bmp` from the given `start_point`. Returns coordinates if
    /// found, or `None` if not. If `rect` is `None`, `bmp.bounds` is used
    /// instead. If `start_point` is `None`, the origin of `rect` is used.
    ///
    /// Tolerance is defined as a float in the range from 0 to 1, where 0 is an
    /// exact match and 1 matches anything.
    fn find_color(
        &self,
        color: u32,
        tolerance: Option<f64>,
        rect: Option<((f64, f64), (f64, f64))>,
        start_point: Option<(f64, f64)>,
    ) -> PyResult<Option<(f64, f64)>> {
        let split_color = hex_to_rgb(color);
        let rgb = Rgba([split_color.0, split_color.1, split_color.2, 255]);
        let rect: Option<Rect> =
            rect.map(|r| Rect::new(Point::new((r.0).0, (r.0).1), Size::new((r.1).0, (r.1).1)));
        let start_point: Option<Point> = start_point.map(|p| Point::new(p.0, p.1));
        if let Some(point) = self.bitmap.find_color(rgb, tolerance, rect, start_point) {
            Ok(Some((point.x, point.y)))
        } else {
            Ok(None)
        }
    }

    /// Returns list of all `(x, y)` coordinates inside `rect` in `bmp`
    /// matching `color` from the given `start_point`. If `rect` is `None`,
    /// `bmp.bounds` is used instead. If `start_point` is `None`, the origin of
    /// `rect` is used.
    fn find_every_color(
        &self,
        color: u32,
        tolerance: Option<f64>,
        rect: Option<((f64, f64), (f64, f64))>,
        start_point: Option<(f64, f64)>,
    ) -> PyResult<Vec<(f64, f64)>> {
        let split_color = hex_to_rgb(color);
        let rgb = Rgba([split_color.0, split_color.1, split_color.2, 255]);
        let rect: Option<Rect> =
            rect.map(|r| Rect::new(Point::new((r.0).0, (r.0).1), Size::new((r.1).0, (r.1).1)));
        let start_point: Option<Point> = start_point.map(|p| Point::new(p.0, p.1));
        let points = self
            .bitmap
            .find_every_color(rgb, tolerance, rect, start_point)
            .iter()
            .map(|p| (p.x, p.y))
            .collect();
        Ok(points)
    }

    /// Returns count of color in bitmap. Functionally equivalent to:
    ///
    /// `len(find_every_color(color, tolerance, rect, start_point))`
    fn count_of_color(
        &self,
        color: u32,
        tolerance: Option<f64>,
        rect: Option<((f64, f64), (f64, f64))>,
        start_point: Option<(f64, f64)>,
    ) -> PyResult<u64> {
        let split_color = hex_to_rgb(color);
        let rgb = Rgba([split_color.0, split_color.1, split_color.2, 255]);
        let rect: Option<Rect> =
            rect.map(|r| Rect::new(Point::new((r.0).0, (r.0).1), Size::new((r.1).0, (r.1).1)));
        let start_point: Option<Point> = start_point.map(|p| Point::new(p.0, p.1));
        let count = self
            .bitmap
            .count_of_color(rgb, tolerance, rect, start_point);
        Ok(count)
    }

    /// Attempts to find `needle` inside `rect` in `bmp` from the given
    /// `start_point`. Returns coordinates if found, or `None` if not. If
    /// `rect` is `None`, `bmp.bounds` is used instead. If `start_point` is
    /// `None`, the origin of `rect` is used.
    ///
    /// Tolerance is defined as a float in the range from 0 to 1, where 0 is an
    /// exact match and 1 matches anything.
    fn find_bitmap(
        &self,
        needle: &Bitmap,
        tolerance: Option<f64>,
        rect: Option<((f64, f64), (f64, f64))>,
        start_point: Option<(f64, f64)>,
    ) -> PyResult<Option<(f64, f64)>> {
        let rect: Option<Rect> =
            rect.map(|r| Rect::new(Point::new((r.0).0, (r.0).1), Size::new((r.1).0, (r.1).1)));
        let start_point: Option<Point> = start_point.map(|p| Point::new(p.0, p.1));
        if let Some(point) = self
            .bitmap
            .find_bitmap(&needle.bitmap, tolerance, rect, start_point)
        {
            Ok(Some((point.x, point.y)))
        } else {
            Ok(None)
        }
    }

    /// Returns list of all `(x, y)` coordinates inside `rect` in `bmp`
    /// matching `needle` from the given `start_point`. If `rect` is `None`,
    /// `bmp.bounds` is used instead. If `start_point` is `None`, the origin of
    /// `rect` is used.
    fn find_every_bitmap(
        &self,
        needle: &Bitmap,
        tolerance: Option<f64>,
        rect: Option<((f64, f64), (f64, f64))>,
        start_point: Option<(f64, f64)>,
    ) -> PyResult<Vec<(f64, f64)>> {
        let rect: Option<Rect> =
            rect.map(|r| Rect::new(Point::new((r.0).0, (r.0).1), Size::new((r.1).0, (r.1).1)));
        let start_point: Option<Point> = start_point.map(|p| Point::new(p.0, p.1));
        let points = self
            .bitmap
            .find_every_bitmap(&needle.bitmap, tolerance, rect, start_point)
            .iter()
            .map(|p| (p.x, p.y))
            .collect();
        Ok(points)
    }

    /// Returns count of occurrences of `needle` in `bmp`. Functionally
    /// equivalent to:
    ///
    /// `len(find_every_bitmap(color, tolerance, rect, start_point))`
    fn count_of_bitmap(
        &self,
        needle: &Bitmap,
        tolerance: Option<f64>,
        rect: Option<((f64, f64), (f64, f64))>,
        start_point: Option<(f64, f64)>,
    ) -> PyResult<u64> {
        let rect: Option<Rect> =
            rect.map(|r| Rect::new(Point::new((r.0).0, (r.0).1), Size::new((r.1).0, (r.1).1)));
        let start_point: Option<Point> = start_point.map(|p| Point::new(p.0, p.1));
        let count = self
            .bitmap
            .count_of_bitmap(&needle.bitmap, tolerance, rect, start_point);
        Ok(count)
    }

    /// Returns new bitmap object created from a portion of another.
    ///
    /// Exceptions:
    ///     - `ValueError` is thrown if the portion was out of bounds.
    fn cropped(&mut self, rect: ((f64, f64), (f64, f64))) -> PyResult<Py<Bitmap>> {
        let rect = Rect::new(
            Point::new((rect.0).0, (rect.0).1),
            Size::new((rect.1).0, (rect.1).1),
        );
        let bmp = self.bitmap.cropped(rect).map_err(FromImageError::from)?;
        let gil = Python::acquire_gil();
        let result = Py::new(gil.python(), Bitmap { bitmap: bmp })?;
        Ok(result)
    }

    /// Returns true if bitmap is equal to receiver with the given tolerance.
    pub fn is_bitmap_equal(&self, bitmap: &Bitmap, tolerance: Option<f64>) -> PyResult<bool> {
        Ok(self.bitmap.bitmap_eq(&bitmap.bitmap, tolerance))
    }

    #[getter(scale)]
    fn scale(&self) -> PyResult<f64> {
        Ok(self.bitmap.scale)
    }

    #[getter(width)]
    fn width(&self) -> PyResult<f64> {
        Ok(self.bitmap.size.width)
    }

    #[getter(height)]
    fn height(&self) -> PyResult<f64> {
        Ok(self.bitmap.size.height)
    }

    #[getter(size)]
    fn size(&self) -> PyResult<(f64, f64)> {
        Ok((self.bitmap.size.width, self.bitmap.size.height))
    }

    #[getter(bounds)]
    fn bounds(&self) -> PyResult<((f64, f64), (f64, f64))> {
        let bounds = self.bitmap.bounds();
        let result = (
            (bounds.origin.x, bounds.origin.y),
            (bounds.size.width, bounds.size.height),
        );
        Ok(result)
    }
}

/// Returns a screengrab of the given portion of the main display, or the
/// entire display if `rect` is `None`. The `rect` parameter is in the form of
/// `((x, y), (width, height))`.
///
/// Exceptions:
///     - `ValueError` is thrown if the rect is out of bounds.
///     - `IOError` is thrown if the image failed to parse.
#[pyfunction]
fn capture_screen(python: Python, rect: Option<((f64, f64), (f64, f64))>) -> PyResult<Py<Bitmap>> {
    let result: ImageResult<autopilot::bitmap::Bitmap> = if let Some(rect) = rect {
        let portion = Rect::new(
            Point::new((rect.0).0, (rect.0).1),
            Size::new((rect.1).0, (rect.1).1),
        );
        autopilot::bitmap::capture_screen_portion(portion)
    } else {
        autopilot::bitmap::capture_screen()
    };
    let bmp = result.map_err(FromImageError::from)?;
    let result = Py::new(python, Bitmap { bitmap: bmp })?;
    Ok(result)
}

/// This module defines the class `Bitmap` for accessing bitmaps and searching
/// for bitmaps on-screen.
///
/// It also defines functions for taking screenshots of the screen.
#[pymodule(bitmap)]
fn init(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Bitmap>()?;
    m.add_wrapped(wrap_pyfunction!(capture_screen))?;
    Ok(())
}

enum AutoPyImageFormat {
    BMP,
    GIF,
    JPEG,
    PNG,
    Unsupported,
}

impl From<AutoPyImageFormat> for ImageOutputFormat {
    fn from(format: AutoPyImageFormat) -> ImageOutputFormat {
        use image::ImageOutputFormat::*;
        match format {
            AutoPyImageFormat::BMP => BMP,
            AutoPyImageFormat::GIF => GIF,
            AutoPyImageFormat::JPEG => JPEG(100),
            AutoPyImageFormat::PNG => PNG,
            AutoPyImageFormat::Unsupported => {
                Unsupported("This image format is unsupported by AutoPy".to_string())
            }
        }
    }
}

impl From<ImageOutputFormat> for AutoPyImageFormat {
    fn from(format: ImageOutputFormat) -> AutoPyImageFormat {
        use image::ImageOutputFormat::*;
        match format {
            BMP => AutoPyImageFormat::BMP,
            GIF => AutoPyImageFormat::GIF,
            JPEG(_) => AutoPyImageFormat::JPEG,
            PNG => AutoPyImageFormat::PNG,
            _ => AutoPyImageFormat::Unsupported,
        }
    }
}

fn image_output_format_from_extension(extension: &str) -> AutoPyImageFormat {
    let extension: &str = &(extension.to_lowercase());
    match extension {
        "bmp" => AutoPyImageFormat::BMP,
        "gif" => AutoPyImageFormat::GIF,
        "jpeg" => AutoPyImageFormat::JPEG,
        "png" => AutoPyImageFormat::PNG,
        _ => AutoPyImageFormat::Unsupported,
    }
}
