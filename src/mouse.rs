// Copyright 2018, 2019 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use autopilot::geometry::Point;
use pyo3::prelude::*;

struct FromMouseError(autopilot::mouse::MouseError);

// NB: pyo3 doesn't currently support static properties for python classes, so
// using a separate class as a namespace instead.
#[pyclass]
/// Constants used by this module in order to specify mouse buttons.
struct _Button {}

#[pyclass]
/// Constants used by this module in order to specify mouse buttons.
struct Button {
    button: autopilot::mouse::Button,
}

/// Moves the mouse to the given `(x, y)` coordinate.
///
/// Exceptions:
///     - `ValueError` is thrown if the point is out of index.
#[pyfunction]
fn move_py(x: f64, y: f64) -> PyResult<()> {
    let result = autopilot::mouse::move_to(Point::new(x, y));
    result.map_err(FromMouseError::from)?;
    Ok(())
}

#[pymethods]
impl _Button {
    #[getter(LEFT)]
    fn left(&self) -> PyResult<Py<Button>> {
        self.init_button_ref(autopilot::mouse::Button::Left)
    }

    #[getter(RIGHT)]
    fn right(&self) -> PyResult<Py<Button>> {
        self.init_button_ref(autopilot::mouse::Button::Right)
    }

    #[getter(MIDDLE)]
    fn middle(&self) -> PyResult<Py<Button>> {
        self.init_button_ref(autopilot::mouse::Button::Middle)
    }
}

/// Returns a tuple `(x, y)` of the current mouse position.
#[pyfunction]
fn location() -> PyResult<(f64, f64)> {
    let point = autopilot::mouse::location();
    Ok((point.x, point.y))
}

/// Holds down or releases the given mouse button in the current position.
/// Button can be `LEFT`, `RIGHT`, `MIDDLE`, or `None` to default to the
/// left button.
#[pyfunction]
fn toggle(button: Option<&Button>, down: bool) -> PyResult<()> {
    use autopilot::mouse::Button::*;
    autopilot::mouse::toggle(button.map_or(Left, |x| x.button), down);
    Ok(())
}

/// Convenience wrapper around `toggle()` that holds down and then releases
/// the given mouse button. By default, the left button is pressed.
#[pyfunction]
fn click(button: Option<&Button>, delay: Option<f64>) -> PyResult<()> {
    let delay_ms: Option<u64> = delay.map(|x| x as u64 * 1000);
    use autopilot::mouse::Button::*;
    autopilot::mouse::click(button.map_or(Left, |x| x.button), delay_ms);
    Ok(())
}

/// Smoothly moves the mouse to the given `(x, y)` coordinate in a straight
/// line.
///
/// Exceptions:
///     - `ValueError` is thrown if the point is out of index.
#[pyfunction]
fn smooth_move(x: f64, y: f64, duration: Option<f64>) -> PyResult<()> {
    let result = autopilot::mouse::smooth_move(Point::new(x, y), duration);
    result.map_err(FromMouseError::from)?;
    Ok(())
}

/// This module contains functions for getting the current state of and
/// controlling the mouse cursor.
///
/// Unless otherwise stated, coordinates are those of a screen coordinate
/// system, where the origin is at the top left.
#[pymodule(mouse)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    // Workaround bug where #[pyfunction(m, "move")] identifier causes error in pyo3.
    m.add("move", wrap_pyfunction!(move_py)(py))?;
    m.add_wrapped(wrap_pyfunction!(location))?;
    m.add_wrapped(wrap_pyfunction!(toggle))?;
    m.add_wrapped(wrap_pyfunction!(click))?;
    m.add_wrapped(wrap_pyfunction!(smooth_move))?;

    m.add("Button", Py::new(py, _Button {})?)?;
    Ok(())
}

impl _Button {
    fn init_button_ref(&self, button: autopilot::mouse::Button) -> PyResult<Py<Button>> {
        let gil = Python::acquire_gil();
        let result = Py::new(gil.python(), Button { button: button })?;
        Ok(result)
    }
}

impl From<autopilot::mouse::MouseError> for FromMouseError {
    fn from(err: autopilot::mouse::MouseError) -> FromMouseError {
        FromMouseError { 0: err }
    }
}

impl From<FromMouseError> for PyErr {
    fn from(err: FromMouseError) -> PyErr {
        use autopilot::mouse::MouseError::*;
        match err.0 {
            OutOfBounds => pyo3::exceptions::ValueError::py_err("Point out of bounds"),
        }
    }
}
