use crate::data::EditorData;
use crate::message::EditorMessage;
use iced::{pick_list, Column, HorizontalAlignment, Length, PickList, Row, Text};
use texture_generation::utils::resource::{Resource, ResourceManager};

pub mod door;
pub mod furniture;
pub mod map;
pub mod tile;
pub mod tools;
pub mod wall;
pub mod window;

pub trait Tool {
    fn get_name(&self) -> &str;

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool;

    fn view_sidebar(&mut self, data: &EditorData) -> Column<EditorMessage>;
}

fn create_pick_list<'a, T: Resource>(
    resource_manager: &ResourceManager<T>,
    state: &'a mut pick_list::State<String>,
    id: usize,
    on_selected: fn(String) -> EditorMessage,
) -> PickList<'a, String, EditorMessage> {
    let selected_name = resource_manager.get(id).get_name();
    let names: Vec<String> = resource_manager
        .get_names()
        .iter()
        .map(|n| n.to_string())
        .collect();
    PickList::new(state, names, Some(selected_name.to_string()), on_selected)
}

pub fn title(label: &str) -> Text {
    Text::new(label)
        .width(Length::Fill)
        .horizontal_alignment(HorizontalAlignment::Center)
}

pub fn help(label: &str) -> Row<EditorMessage> {
    Row::new().push(Text::new("+")).push(Text::new(label))
}
