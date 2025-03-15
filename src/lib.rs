// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// #![feature(specialization, const_fn)]

extern crate autopilot;
extern crate either;
extern crate image;
extern crate pyo3;

pub mod alert;
pub mod bitmap;
pub mod color;
mod internal;
pub mod key;
pub mod mouse;
pub mod screen;

use pyo3::prelude::*;

#[pymodule]
fn autopy(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {

    let alert_module = PyModule::new(m.py(), "alert")?;
    alert::alert(m.py(), &alert_module)?;
    m.add_submodule(&alert_module)?;

    let bitmap_module = PyModule::new(m.py(), "bitmap")?;
    bitmap::bitmap(m.py(), &bitmap_module)?;
    m.add_submodule(&bitmap_module)?;

    let color_module = PyModule::new(m.py(), "color")?;
    color::color(m.py(), &color_module)?;
    m.add_submodule(&color_module)?;

    let key_module = PyModule::new(m.py(), "key")?;
    key::key(m.py(), &key_module)?;
    m.add_submodule(&key_module)?;

    let mouse_module = PyModule::new(m.py(), "mouse")?;
    mouse::mouse(m.py(), &mouse_module)?;
    m.add_submodule(&mouse_module)?;

    let screen_module = PyModule::new(m.py(), "screen")?;
    screen::screen(m.py(), &screen_module)?;
    m.add_submodule(&screen_module)?;

    Ok(())
}
