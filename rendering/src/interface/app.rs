use super::rendering::{Initialization, Renderer};
use crate::interface::input::{KeyCode, MouseButton};

/// A trait to handle simple applications like the examples.
pub trait App {
    /// Initializes the application.
    fn init(&mut self, _initialization: &mut dyn Initialization) {}

    /// Renders the application.
    fn render(&mut self, renderer: &mut dyn Renderer);

    /// Handles keyboard input
    fn on_key_released(&mut self, _key: KeyCode) {}

    /// Handles mouse input
    fn on_button_released(&mut self, _button: MouseButton, _point: (u32, u32)) {}
}
