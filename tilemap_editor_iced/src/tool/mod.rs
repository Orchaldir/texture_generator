use crate::data::EditorData;
use crate::message::EditorMessage;
use iced::Element;

pub mod tile;
pub mod tools;
pub mod wall;

pub trait Tool {
    fn get_name(&self) -> &str;

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool;

    fn view_sidebar(&mut self, data: &EditorData) -> Element<EditorMessage>;
}
