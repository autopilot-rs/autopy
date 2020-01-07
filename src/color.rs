// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use internal::rgb_to_hex;
use pyo3::prelude::*;

/// Returns hexadecimal value of given RGB tuple. `r`, `g`, and `b` must be
/// in the range 0 - 255.
#[pyfunction]
fn py_rgb_to_hex(red: u8, green: u8, blue: u8) -> PyResult<u32> {
    Ok(rgb_to_hex(red, green, blue))
}

/// Returns a tuple `(r, g, b)` of the RGB integer values equivalent to the
/// given RGB hexadecimal value. `r`, `g`, and `b` are in the range 0 - 255.
#[pyfunction]
fn hex_to_rgb(hex: u32) -> PyResult<(u8, u8, u8)> {
    let red: u8 = ((hex >> 16) & 0xff) as u8;
    let green: u8 = ((hex >> 8) & 0xff) as u8;
    let blue: u8 = (hex & 0xff) as u8;
    Ok((red, green, blue))
}

/// This module provides functions for converting between the hexadecimal format
/// used by autopy methods and other more readable formats (e.g., RGB tuples).
#[pymodule(color)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("rgb_to_hex", wrap_pyfunction!(py_rgb_to_hex)(py))?;
    m.add_wrapped(wrap_pyfunction!(hex_to_rgb))?;
    Ok(())
}
