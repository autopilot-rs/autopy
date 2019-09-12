use image::ImageError;
use pyo3::prelude::*;

pub struct FromImageError(ImageError);
pub fn rgb_to_hex(red: u8, green: u8, blue: u8) -> u32 {
    ((red as u32) << 16) | ((green as u32) << 8) | blue as u32
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
