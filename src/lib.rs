#![feature(
    proc_macro, proc_macro_attribute, specialization, const_fn, const_align_of, const_size_of
)]

extern crate autopilot;
extern crate either;
extern crate image;
extern crate pyo3;

mod internal;
pub mod alert;
pub mod bitmap;
pub mod color;
pub mod key;
pub mod mouse;
pub mod screen;
