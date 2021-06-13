use iced_native::keyboard::KeyCode;
use iced_native::mouse::Button;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    ChangeTool(usize),
    ChangeTexture(String),
    ChangeWall(String),
    ChangeDoor(String),
    ChangeWindow(String),
    ChangeMapWidth(u32),
    ChangeMapHeight(u32),
    ClickedButton { x: u32, y: u32, button: Button },
    NewMap,
    PressedKey(KeyCode),
    Render,
}
