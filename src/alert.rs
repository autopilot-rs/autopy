extern crate autopilot;
extern crate pyo3;
use autopilot::alert::Response;
use pyo3::prelude::*;

/// This module contains functions for displaying alerts.
#[py::modinit(alert)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    /// Displays alert with the given attributes. If `cancel_button` is not
    /// given, only the default button is displayed. Returns `True` if the
    /// default button was pressed, or `False` if cancelled. Note that the
    /// arguments are keywords, and can be passed as named parameters (e.g.,
    /// `alert(msg="bar", title="foo")`).
    ///
    /// NOTE: Due to limitations in the Win32 API, Windows currently replaces
    /// `default_button` with "OK" and `cancel_button` (if given) with "Cancel".
    /// This may be fixed in a later release.
    #[pyfn(m, "alert")]
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

    Ok(())
}
