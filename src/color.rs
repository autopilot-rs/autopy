// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::internal::{rgb_to_hex, hex_to_rgb};
use pyo3::prelude::*;

/// Returns hexadecimal value of given RGB tuple. `r`, `g`, and `b` must be in
/// the range 0 - 255.
#[pyfunction]
fn py_rgb_to_hex(red: u8, green: u8, blue: u8) -> PyResult<u32> {
    Ok(rgb_to_hex(red, green, blue))
}

/// Returns a tuple `(r, g, b)` of the RGB integer values equivalent to the
/// given RGB hexadecimal value. `r`, `g`, and `b` are in the range 0 - 255.
#[pyfunction]
fn py_hex_to_rgb(hex: u32) -> PyResult<(u8, u8, u8)> {
    Ok(hex_to_rgb(hex))
}

/// This module provides functions for converting between the hexadecimal
/// format used by autopy methods and other more readable formats (e.g., RGB
/// tuples).
#[pymodule]
pub fn color(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("rgb_to_hex", wrap_pyfunction!(py_rgb_to_hex)(py)?)?;
    m.add("hex_to_rgb", wrap_pyfunction!(py_hex_to_rgb)(py)?)?;
    Ok(())
}
