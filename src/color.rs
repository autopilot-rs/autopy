use pyo3::prelude::*;

/// This module provides functions for converting between the hexadecimal format
/// used by autopy methods and other more readable formats (e.g., RGB tuples).
#[pymodinit(color)]
fn init(_py: Python, m: &PyModule) -> PyResult<()> {
    /// Returns hexadecimal value of given RGB tuple. `r`, `g`, and `b` must be
    /// in the range 0 - 255.
    #[pyfn(m, "rgb_to_hex")]
    fn rgb_to_hex(red: u8, green: u8, blue: u8) -> PyResult<u32> {
        Ok(((red as u32) << 16) | ((green as u32) << 8) | blue as u32)
    }

    /// Returns a tuple `(r, g, b)` of the RGB integer values equivalent to the
    /// given RGB hexadecimal value. `r`, `g`, and `b` are in the range 0 - 255.
    #[pyfn(m, "hex_to_rgb")]
    fn hex_to_rgb(hex: u32) -> PyResult<(u8, u8, u8)> {
        let red: u8 = ((hex >> 16) & 0xff) as u8;
        let green: u8 = ((hex >> 8) & 0xff) as u8;
        let blue: u8 = (hex & 0xff) as u8;
        Ok((red, green, blue))
    }

    Ok(())
}
