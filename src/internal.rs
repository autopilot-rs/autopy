// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use image::ImageError;
use pyo3::prelude::*;

pub struct FromImageError(ImageError);

pub fn rgb_to_hex(red: u8, green: u8, blue: u8) -> u32 {
    ((red as u32) << 16) | ((green as u32) << 8) | blue as u32
}

pub fn hex_to_rgb(hex: u32) -> (u8, u8, u8) {
    let red: u8 = ((hex >> 16) & 0xff) as u8;
    let green: u8 = ((hex >> 8) & 0xff) as u8;
    let blue: u8 = (hex & 0xff) as u8;
    (red, green, blue)
}

impl From<ImageError> for FromImageError {
    fn from(err: ImageError) -> FromImageError {
        FromImageError { 0: err }
    }
}

impl From<FromImageError> for PyErr {
    fn from(err: FromImageError) -> PyErr {
        match err.0 {
            ImageError::DimensionError => {
                pyo3::exceptions::ValueError::py_err(format!("{}", err.0))
            }
            _ => pyo3::exceptions::IOError::py_err(format!("{}", err.0)),
        }
    }
}
