use iced_native::keyboard::KeyCode;
use iced_native::mouse::Button;

#[derive(Debug, Clone, Copy)]
pub enum EditorMessage {
    ClickedButton { x: u32, y: u32, button: Button },
    PressedKey(KeyCode),
}
