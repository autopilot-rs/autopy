#![feature(specialization, const_fn)]

extern crate autopilot;
extern crate either;
extern crate image;
#[macro_use]
extern crate pyo3;

pub mod alert;
pub mod bitmap;
pub mod color;
mod internal;
pub mod key;
pub mod mouse;
pub mod screen;
