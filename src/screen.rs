// Copyright 2018, 2019 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use autopilot::geometry::Point;
use image::Pixel;
use internal::{rgb_to_hex, FromImageError};
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

/// Returns hexadecimal value describing the color at a given point.
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
fn get_color(x: f64, y: f64) -> PyResult<u32> {
    let point = Point::new(x, y);
    let rgb = autopilot::screen::get_color(point).map_err(FromImageError::from)?;
    let (r, g, b, _) = rgb.channels4();
    Ok(rgb_to_hex(r, g, b))
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
