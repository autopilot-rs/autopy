use pyo3::prelude::*;
use image::ImageError;

pub struct FromImageError(ImageError);

impl From<ImageError> for FromImageError {
    fn from(err: ImageError) -> FromImageError {
        FromImageError { 0: err }
    }
}

impl From<FromImageError> for PyErr {
    fn from(err: FromImageError) -> PyErr {
        exc::ValueError::new(format!("{}", err.0))
    }
}
