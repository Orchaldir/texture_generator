/// Represents a color with the RGB color model.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/RGB_color_model).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Returns a new color
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Returns a new gray color
    pub const fn gray(value: u8) -> Color {
        Color {
            r: value,
            g: value,
            b: value,
        }
    }

    /// Returns the red component
    ///
    /// ```
    /// use texture_generator::math::color::Color;
    /// assert_eq!(Color::from_rgb(0, 1, 2).r(), 0);
    /// ```
    pub fn r(&self) -> u8 {
        self.r
    }

    /// Returns the green component
    ///
    /// ```
    ///# use texture_generator::math::color::Color;
    /// assert_eq!(Color::from_rgb(0, 1, 2).g(), 1);
    /// ```
    pub fn g(&self) -> u8 {
        self.g
    }

    /// Returns the blue component
    ///
    /// ```
    ///# use texture_generator::math::color::Color;
    /// assert_eq!(Color::from_rgb(0, 1, 2).b(), 2);
    /// ```
    pub fn b(&self) -> u8 {
        self.b
    }
}

impl Default for Color {
    fn default() -> Self {
        PINK
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
