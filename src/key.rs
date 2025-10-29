// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use either::{Either, Left, Right};
use pyo3::prelude::*;
use pyo3::types::PyString;

// NB: pyo3 doesn't currently support static properties for python classes, so
// using a separate class as a namespace instead.
#[pyclass]
/// Constants used by this module in order to specify modifier flags.
struct _Modifier {}

#[pyclass]
/// Constants used by this module in order to specify key codes.
struct _Code {}

#[pyclass]
/// Constants used by this module in order to specify modifier flags.
struct Modifier {
    flag: autopilot::key::Flag,
}

#[pyclass]
/// Constants used by this module in order to specify key codes.
struct Code {
    code: autopilot::key::KeyCode,
}

#[pymethods]
impl _Modifier {
    /// Equivalent to the command key modifier on macOS, the Windows key
    /// modifier on Windows, or the meta key modifiers on X11.
    #[getter(META)]
    fn meta(&self) -> PyResult<Py<Modifier>> {
        self.init_modifier_ref(autopilot::key::Flag::Meta)
    }
    #[getter(ALT)]
    fn alt(&self) -> PyResult<Py<Modifier>> {
        self.init_modifier_ref(autopilot::key::Flag::Alt)
    }
    #[getter(CONTROL)]
    fn control(&self) -> PyResult<Py<Modifier>> {
        self.init_modifier_ref(autopilot::key::Flag::Control)
    }
    #[getter(SHIFT)]
    fn shift(&self) -> PyResult<Py<Modifier>> {
        self.init_modifier_ref(autopilot::key::Flag::Shift)
    }
}

#[pymethods]
impl _Code {
    #[getter(F1)]
    fn f1(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F1)
    }
    #[getter(F2)]
    fn f2(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F2)
    }
    #[getter(F3)]
    fn f3(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F3)
    }
    #[getter(F4)]
    fn f4(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F4)
    }
    #[getter(F5)]
    fn f5(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F5)
    }
    #[getter(F6)]
    fn f6(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F6)
    }
    #[getter(F7)]
    fn f7(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F7)
    }
    #[getter(F8)]
    fn f8(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F8)
    }
    #[getter(F9)]
    fn f9(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F9)
    }
    #[getter(F10)]
    fn f10(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F10)
    }
    #[getter(F11)]
    fn f11(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F11)
    }
    #[getter(F12)]
    fn f12(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F12)
    }
    #[getter(F13)]
    fn f13(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F13)
    }
    #[getter(F14)]
    fn f14(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F14)
    }
    #[getter(F15)]
    fn f15(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F15)
    }
    #[getter(F16)]
    fn f16(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F16)
    }
    #[getter(F17)]
    fn f17(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F17)
    }
    #[getter(F18)]
    fn f18(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F18)
    }
    #[getter(F19)]
    fn f19(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F19)
    }
    #[getter(F20)]
    fn f20(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F20)
    }
    #[getter(F21)]
    fn f21(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F21)
    }
    #[getter(F22)]
    fn f22(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F22)
    }
    #[getter(F23)]
    fn f23(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F23)
    }
    #[getter(F24)]
    fn f24(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::F24)
    }
    #[getter(LEFT_ARROW)]
    fn left_arrow(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::LeftArrow)
    }
    #[getter(CONTROL)]
    fn control(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Control)
    }
    #[getter(RIGHT_ARROW)]
    fn right_arrow(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::RightArrow)
    }
    #[getter(DOWN_ARROW)]
    fn down_arrow(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::DownArrow)
    }
    #[getter(END)]
    fn end(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::End)
    }
    #[getter(UP_ARROW)]
    fn up_arrow(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::UpArrow)
    }
    #[getter(PAGE_UP)]
    fn page_up(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::PageUp)
    }
    #[getter(ALT)]
    fn alt(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Alt)
    }
    #[getter(RETURN)]
    fn return_code(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Return)
    }
    #[getter(PAGE_DOWN)]
    fn page_down(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::PageDown)
    }
    #[getter(DELETE)]
    fn delete(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Delete)
    }
    #[getter(HOME)]
    fn home(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Home)
    }
    #[getter(ESCAPE)]
    fn escape(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Escape)
    }
    #[getter(BACKSPACE)]
    fn backspace(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Backspace)
    }
    #[getter(SPACE)]
    fn space(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Space)
    }
    #[getter(META)]
    fn meta(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Meta)
    }
    #[getter(CAPS_LOCK)]
    fn caps_lock(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::CapsLock)
    }
    #[getter(SHIFT)]
    fn shift(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Shift)
    }
    #[getter(TAB)]
    fn tab(&self) -> PyResult<Py<Code>> {
        self.init_code_ref(autopilot::key::KeyCode::Tab)
    }
}

/// Holds down the given key or keycode if `down` is `True`, or releases it if
/// not. Integer keycodes and modifiers should be taken from module constants
/// (e.g., `Code.DELETE` or `Modifier.META`). If the given key is a character,
/// it is automatically converted to a keycode corresponding to the current
/// keyboard layout.
#[pyfunction]
#[pyo3(signature = (key, down, modifiers=None, modifier_delay=None))]
fn toggle(
    py: Python<'_>,
    key: &Bound<'_, PyAny>,
    down: bool,
    modifiers: Option<Vec<Py<Modifier>>>,
    modifier_delay: Option<u64>,
) -> PyResult<()> {
    let modifier_delay_ms: u64 = modifier_delay.map(|x| x as u64 * 1000).unwrap_or(0);
    if let Some(either) = py_object_to_key_code_convertible(key) {
        let flags: Vec<_> = modifiers
            .unwrap_or(Vec::new())
            .iter()
            .map(|x| x.borrow(py).flag)
            .collect();
        match either {
            Left(x) => autopilot::key::toggle(&x, down, &flags, modifier_delay_ms),
            Right(x) => autopilot::key::toggle(&x, down, &flags, modifier_delay_ms),
        };
        Ok(())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected string or key code",
        ))
    }
}

/// Convenience wrapper around `toggle()` that holds down and then releases the
/// given key and modifiers.
#[pyfunction]
#[pyo3(signature = (key, modifiers=None, delay=None, modifier_delay=None))]
fn tap(
    py: Python<'_>,
    key: &Bound<'_, PyAny>,
    modifiers: Option<Vec<Py<Modifier>>>,
    delay: Option<f64>,
    modifier_delay: Option<f64>,
) -> PyResult<()> {
    let delay_ms: u64 = delay.map(|x| x as u64 * 1000).unwrap_or(0);
    let modifier_delay_ms: u64 = modifier_delay.map(|x| x as u64 * 1000).unwrap_or(delay_ms);
    if let Some(either) = py_object_to_key_code_convertible(key) {
        let flags: Vec<_> = modifiers
            .unwrap_or(Vec::new())
            .iter()
            .map(|x| x.borrow(py).flag)
            .collect();
        match either {
            Left(x) => autopilot::key::tap(&x, &flags, delay_ms, modifier_delay_ms),
            Right(x) => autopilot::key::tap(&x, &flags, delay_ms, modifier_delay_ms),
        };
        Ok(())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected string or key code",
        ))
    }
}

/// Attempts to simulate typing a string at the given WPM, or as fast as
/// possible if the WPM is 0.
#[pyfunction]
#[pyo3(signature = (string, wpm=None))]
fn type_string(string: &str, wpm: Option<f64>) -> PyResult<()> {
    autopilot::key::type_string(string, &[], wpm.unwrap_or(0.0), 0.0);
    Ok(())
}

/// This module contains functions for controlling the keyboard.
#[pymodule]
pub fn key(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("Modifier", Py::new(py, _Modifier {})?)?;
    m.add("Code", Py::new(py, _Code {})?)?;
    m.add_wrapped(wrap_pyfunction!(toggle))?;
    m.add_wrapped(wrap_pyfunction!(tap))?;
    m.add_wrapped(wrap_pyfunction!(type_string))?;
    Ok(())
}

fn py_object_to_key_code_convertible(
    object: &Bound<'_, PyAny>,
) -> Option<Either<autopilot::key::Code, autopilot::key::Character>> {
    // object.extract::<Bound<'_, PyAny>>
    if let Ok(code) = object.cast::<Code>() {
        return Some(Left(autopilot::key::Code(code.borrow().code)));
    } else if let Ok(key) = object.cast::<PyString>() {
        if let Some(c) = key.to_string().chars().next() {
            return Some(Right(autopilot::key::Character(c)));
        }
    }
    None
}

impl _Modifier {
    fn init_modifier_ref(&self, flag: autopilot::key::Flag) -> PyResult<Py<Modifier>> {
        Python::attach(|py| {
            let result = Py::new(py, Modifier { flag: flag })?;
            Ok(result)
        })
    }
}

impl _Code {
    fn init_code_ref(&self, code: autopilot::key::KeyCode) -> PyResult<Py<Code>> {
        Python::attach(|py| {
            let result = Py::new(py, Code { code: code })?;
            Ok(result)
        })
    }
}
