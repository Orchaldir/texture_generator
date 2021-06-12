use crate::data::EditorData;
use crate::message::EditorMessage;

pub mod tile;

pub trait Tool {
    fn get_name(&self) -> &str;

    fn update(&mut self, data: &EditorData, message: EditorMessage);
}
