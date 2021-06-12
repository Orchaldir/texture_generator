use crate::data::EditorData;
use crate::message::EditorMessage;

pub mod tile;
pub mod tools;

pub trait Tool {
    fn get_name(&self) -> &str;

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool;
}
