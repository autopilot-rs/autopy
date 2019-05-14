use either::{Either, Left, Right};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyString};

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
    /// Equivalent to the command key modifier on Mac OS X, the Windows key
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
}

/// Holds down the given key or keycode if `down` is `True`, or releases it
/// if not. Integer keycodes and modifiers should be taken from module
/// constants (e.g., `key.K_DELETE` or `key.MOD_META`). If the given key is
/// a character, it is automatically converted to a keycode corresponding to
/// the current keyboard layout.
#[pyfunction]
fn toggle(
    key: &PyAny,
    down: bool,
    modifiers: Vec<&Modifier>,
    modifier_delay: Option<u64>,
) -> PyResult<()> {
    let modifier_delay_ms: u64 = modifier_delay.map(|x| x as u64 * 1000).unwrap_or(0);
    if let Some(either) = py_object_to_key_code_convertible(key) {
        let flags: Vec<_> = modifiers.iter().map(|x| x.flag).collect();
        match either {
            Left(x) => autopilot::key::toggle(&x, down, &flags, modifier_delay_ms),
            Right(x) => autopilot::key::toggle(&x, down, &flags, modifier_delay_ms),
        };
        Ok(())
    } else {
        Err(pyo3::exceptions::TypeError::py_err(
            "Expected string or key code",
        ))
    }
}

/// Convenience wrapper around `toggle()` that holds down and then releases
/// the given key and modifiers.
#[pyfunction]
fn tap(key: &PyAny, modifiers: Vec<&Modifier>, delay: Option<f64>) -> PyResult<()> {
    let delay_ms: u64 = delay.map(|x| x as u64 * 1000).unwrap_or(0);
    if let Some(either) = py_object_to_key_code_convertible(key) {
        let flags: Vec<_> = modifiers.iter().map(|x| x.flag).collect();
        match either {
            Left(x) => autopilot::key::tap(&x, &flags, delay_ms),
            Right(x) => autopilot::key::tap(&x, &flags, delay_ms),
        };
        Ok(())
    } else {
        Err(pyo3::exceptions::TypeError::py_err(
            "Expected string or key code",
        ))
    }
}

/// Attempts to simulate typing a string at the given WPM, or as fast as
/// possible if the WPM is 0.
#[pyfunction]
fn type_string(string: &str, wpm: Option<f64>) -> PyResult<()> {
    autopilot::key::type_string(string, &[], wpm.unwrap_or(0.0), 0.0);
    Ok(())
}

/// This module contains functions for controlling the keyboard.
#[pymodule(key)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    try!(m.add("Modifier", Py::new(py, _Modifier {})?));
    try!(m.add("Code", Py::new(py, _Code {})?));
    m.add_wrapped(wrap_pyfunction!(toggle))?;
    m.add_wrapped(wrap_pyfunction!(tap))?;
    m.add_wrapped(wrap_pyfunction!(type_string))?;
    Ok(())
}

fn py_object_to_key_code_convertible(
    object: &PyAny,
) -> Option<Either<autopilot::key::Code, autopilot::key::Character>> {
    if let Ok(code) = Code::try_from(object.as_ref()) {
        return Some(Left(autopilot::key::Code(code.code)));
    } else if let Ok(key) = PyString::try_from(object.as_ref()) {
        if let Ok(string) = key.to_string() {
            if let Some(c) = string.chars().next() {
                return Some(Right(autopilot::key::Character(c)));
            }
        }
    }
    None
}

impl _Modifier {
    fn init_modifier_ref(&self, flag: autopilot::key::Flag) -> PyResult<Py<Modifier>> {
        let gil = Python::acquire_gil();
        let result = try!(Py::new(gil.python(), Modifier { flag: flag }));
        Ok(result)
    }
}

impl _Code {
    fn init_code_ref(&self, code: autopilot::key::KeyCode) -> PyResult<Py<Code>> {
        let gil = Python::acquire_gil();
        let result = try!(Py::new(gil.python(), Code { code: code }));
        Ok(result)
    }
}
