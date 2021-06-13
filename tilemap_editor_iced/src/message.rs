use iced_native::keyboard::KeyCode;
use iced_native::mouse::Button;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    ChangeDoorStyle(String),
    ChangeFurnitureStyle(String),
    ChangeTexture(String),
    ChangeTool(usize),
    ChangeWallStyle(String),
    ChangeWindowStyle(String),
    ChangeWidth(u32),
    ChangeHeight(u32),
    ClickedButton { x: u32, y: u32, button: Button },
    NewMap,
    PressedKey(KeyCode),
    Render,
}
