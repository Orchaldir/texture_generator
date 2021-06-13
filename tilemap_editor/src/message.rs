use iced_native::keyboard::KeyCode;
use iced_native::mouse::Button;
use tilemap::tilemap::Side;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    ChangeDoorStyle(String),
    ChangeFurnitureStyle(String),
    ChangeHeight(u32),
    ChangeSide(Side),
    ChangeTexture(String),
    ChangeTool(usize),
    ChangeWallStyle(String),
    ChangeWidth(u32),
    ChangeWindowStyle(String),
    ClickedButton { x: u32, y: u32, button: Button },
    NewMap,
    PressedKey(KeyCode),
    Render,
}
