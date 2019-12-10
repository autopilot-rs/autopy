// Copyright 2018, 2019 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use autopilot::alert::Response;
use pyo3::prelude::*;

/// Displays alert with the given attributes. If `cancel_button` is not
/// given, only the default button is displayed. Returns `True` if the
/// default button was pressed, or `False` if cancelled. Note that the
/// arguments are keywords, and can be passed as named parameters (e.g.,
/// `alert(msg='bar', title='foo')`).
///
/// NOTE: Due to limitations in the Win32 API, Windows currently replaces
/// `default_button` with 'OK' and `cancel_button` (if given) with 'Cancel'.
/// This may be fixed in a later release.
#[pyfunction]
fn alert(
    msg: &str,
    title: Option<&str>,
    default_button: Option<&str>,
    cancel_button: Option<&str>,
) -> PyResult<bool> {
    let title = title.unwrap_or("AutoPy Alert");
    let resp = autopilot::alert::alert(msg, Some(title), default_button, cancel_button);
    Ok(match resp {
        Response::Default => true,
        Response::Cancel => false,
    })
}

/// This module contains functions for displaying alerts.
#[pymodule(alert)]
fn init(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(alert))?;

    Ok(())
}
