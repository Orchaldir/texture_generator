use iced_native::keyboard::KeyCode;
use iced_native::mouse::Button;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    ChangeDoorStyle(String),
    ChangeFurnitureStyle(String),
    ChangeMapWidth(u32),
    ChangeMapHeight(u32),
    ChangeTexture(String),
    ChangeTool(usize),
    ChangeWallStyle(String),
    ChangeWindowStyle(String),
    ClickedButton { x: u32, y: u32, button: Button },
    NewMap,
    PressedKey(KeyCode),
    Render,
}
