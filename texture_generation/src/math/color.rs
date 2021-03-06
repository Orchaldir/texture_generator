use crate::math::interpolate::lerp;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, MulAssign};

/// Represents a color with the RGB color model.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/RGB_color_model).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Returns a new color.
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Converts a string to a color, if possible:
    ///
    /// ```
    /// use texture_generation::math::color::{Color, ORANGE};
    /// assert_eq!(Color::convert("#FFA500").unwrap(), ORANGE);
    /// ```
    pub fn convert(hex_code: &str) -> Result<Color> {
        if !hex_code.starts_with('#') {
            bail!("'{}' needs to start with # to be a color", hex_code);
        } else if hex_code.len() != 7 {
            bail!("'{}' needs to be 7 characters long to be a color", hex_code);
        }

        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16).context(format!(
            "Failed to parse the value of red from '{}'",
            hex_code
        ))?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16).context(format!(
            "Failed to parse the value of green from '{}'",
            hex_code
        ))?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16).context(format!(
            "Failed to parse the value of blue from '{}'",
            hex_code
        ))?;

        Ok(Color::from_rgb(r, g, b))
    }

    /// Returns a new gray color.
    pub const fn gray(value: u8) -> Color {
        Color {
            r: value,
            g: value,
            b: value,
        }
    }

    /// Returns the red component.
    ///
    /// ```
    /// use texture_generation::math::color::Color;
    /// assert_eq!(Color::from_rgb(0, 1, 2).r(), 0);
    /// ```
    pub fn r(&self) -> u8 {
        self.r
    }

    /// Returns the green component
    ///
    /// ```
    ///# use texture_generation::math::color::Color;
    /// assert_eq!(Color::from_rgb(0, 1, 2).g(), 1);
    /// ```
    pub fn g(&self) -> u8 {
        self.g
    }

    /// Returns the blue component.
    ///
    /// ```
    ///# use texture_generation::math::color::Color;
    /// assert_eq!(Color::from_rgb(0, 1, 2).b(), 2);
    /// ```
    pub fn b(&self) -> u8 {
        self.b
    }

    /// Interpolates linearly with another color.
    ///
    /// ```
    /// use texture_generation::math::color::Color;
    /// let color0 = Color::from_rgb(  0, 25, 120);
    /// let color1 = Color::from_rgb(200, 75, 220);
    /// let result = Color::from_rgb(100, 50, 170);
    ///
    /// assert_eq!(color0.lerp(&color1, 0.5), result);
    /// ```
    pub fn lerp(&self, other: &Color, factor: f32) -> Color {
        Color {
            r: lerp(self.r, other.r, factor),
            g: lerp(self.g, other.g, factor),
            b: lerp(self.b, other.b, factor),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        PINK
    }
}

pub fn convert(colors: &[Color]) -> Vec<u8> {
    let n = colors.len();
    let mut data = Vec::with_capacity(n * 3);

    for color in colors {
        data.push(color.r());
        data.push(color.g());
        data.push(color.b());
    }

    data
}

pub fn convert_bgra(colors: &[Color]) -> Vec<u8> {
    let n = colors.len();
    let mut data = Vec::with_capacity(n * 3);

    for color in colors {
        data.push(color.b());
        data.push(color.g());
        data.push(color.r());
        data.push(255);
    }

    data
}

/// Adds a [`Color`] to another [`Color`].
///
/// ```
///# use texture_generation::math::color::Color;
/// let a = Color::from_rgb(10, 100, 255);
/// let b = Color::from_rgb(5, 80, 100);
/// let result = Color::from_rgb(15, 180, 255);
///
/// assert_eq!(a + b, result);
/// assert_eq!(b + a, result);
/// ```
impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::from_rgb(
            self.r.saturating_add(other.r),
            self.g.saturating_add(other.g),
            self.b.saturating_add(other.b),
        )
    }
}

/// Multiplies a [`Color`] with a float and returns a new color.
///
/// ```
///# use texture_generation::math::color::Color;
/// let vector = Color::from_rgb(0, 100, 255);
///
/// assert_eq!(vector * -1.0, Color::from_rgb(0, 0, 0));
/// assert_eq!(vector * 0.5, Color::from_rgb(0, 50, 127));
/// assert_eq!(vector * 2.0, Color::from_rgb(0, 200, 255));
/// ```
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, value: f32) -> Color {
        Color::from_rgb(
            (self.r as f32 * value) as u8,
            (self.g as f32 * value) as u8,
            (self.b as f32 * value) as u8,
        )
    }
}

/// Multiplies a [`Color`] with a float.
///
/// ```
///# use texture_generation::math::color::Color;
/// let mut vector = Color::from_rgb(0, 100, 255);
///
/// vector *= 0.5;
///
/// assert_eq!(vector, Color::from_rgb(0, 50, 127));
/// ```
impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, factor: f32) {
        self.r = (self.r as f32 * factor) as u8;
        self.g = (self.g as f32 * factor) as u8;
        self.b = (self.b as f32 * factor) as u8;
    }
}

pub const BLACK: Color = Color::from_rgb(0, 0, 0);
pub const BLUE: Color = Color::from_rgb(0, 0, 255);
pub const CYAN: Color = Color::from_rgb(0, 255, 255);
pub const GREEN: Color = Color::from_rgb(0, 255, 0);
pub const MAGENTA: Color = Color::from_rgb(255, 0, 255);
pub const ORANGE: Color = Color::from_rgb(255, 165, 0);
pub const RED: Color = Color::from_rgb(255, 0, 0);
pub const PINK: Color = Color::from_rgb(255, 0, 128);
pub const WHITE: Color = Color::from_rgb(255, 255, 255);
pub const YELLOW: Color = Color::from_rgb(255, 255, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_empty_string() {
        assert!(Color::convert("").is_err());
    }

    #[test]
    fn test_from_string_invalid_start() {
        assert!(Color::convert("FFA500").is_err());
    }

    #[test]
    fn test_from_string_part() {
        assert!(Color::convert("#").is_err());
        assert!(Color::convert("#FF").is_err());
        assert!(Color::convert("#FFA5").is_err());
        assert!(Color::convert("#FFA50").is_err());
    }

    #[test]
    fn test_from_string_ignore_case() {
        assert_eq!(Color::convert("#FFA500").unwrap(), ORANGE);
        assert_eq!(Color::convert("#ffa500").unwrap(), ORANGE);
    }
}
