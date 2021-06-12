use iced_native::mouse::Button;

#[derive(Debug, Clone, Copy)]
pub enum EditorMessage {
    Click { x: u32, y: u32, button: Button },
}
