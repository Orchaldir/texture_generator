use iced_native::keyboard::KeyCode;
use iced_native::mouse::Button;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    ChangeTool(usize),
    ChangeTexture(String),
    ChangeWall(String),
    ChangeDoor(String),
    ClickedButton { x: u32, y: u32, button: Button },
    PressedKey(KeyCode),
    Render,
}
