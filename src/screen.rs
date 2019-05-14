use autopilot::geometry::Point;
use image::Pixel;
use internal::FromImageError;
use pyo3::prelude::*;

/// Returns the scale of the main screen, i.e. how many pixels are in a
/// point.
#[pyfunction]
fn scale() -> PyResult<f64> {
    Ok(autopilot::screen::scale())
}

/// Returns a tuple `(width, height)` of the size of the main screen in
/// points.
#[pyfunction]
fn size() -> PyResult<(f64, f64)> {
    let size = autopilot::screen::size();
    Ok((size.width, size.height))
}

/// Returns `True` if the given point is inside the main screen boundaries.
#[pyfunction]
fn is_point_visible(x: f64, y: f64) -> PyResult<bool> {
    Ok(autopilot::screen::is_point_visible(Point::new(x, y)))
}

/// Returns `(r, g, b)` tuple describing the color at a given point.
///
/// Functionally equivalent to:
///
///     rect = ((x, y), (1, 1))
///     bitmap.capture_screen_portion(rect).get_color(0, 0)
///
/// only more efficient/convenient.
///
/// Exceptions:
///     - `ValueError` is thrown if the point out of bounds.
#[pyfunction]
fn get_color(x: f64, y: f64) -> PyResult<(u8, u8, u8)> {
    let point = Point::new(x, y);
    let rgb = try!(autopilot::screen::get_color(point).map_err(FromImageError::from));
    let (r, g, b, _) = rgb.channels4();
    Ok((r, g, b))
}

/// This module contains functions for working with the screen.
#[pymodule(screen)]
fn init(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(scale))?;
    m.add_wrapped(wrap_pyfunction!(size))?;
    m.add_wrapped(wrap_pyfunction!(is_point_visible))?;
    m.add_wrapped(wrap_pyfunction!(get_color))?;
    Ok(())
}
