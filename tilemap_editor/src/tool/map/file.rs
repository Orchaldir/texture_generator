use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{help, title, Tool};
use iced::{button, Button, Column, Text};

#[derive(Clone, Debug, Default)]
pub struct FileTool {
    reload_state: button::State,
    save_state: button::State,
    export_state: button::State,
}

impl Tool for FileTool {
    fn get_name(&self) -> &str {
        "File"
    }

    fn update(&mut self, _data: &mut EditorData, _message: EditorMessage) -> bool {
        false
    }

    fn view_sidebar(&mut self, _data: &EditorData) -> Column<EditorMessage> {
        let reload_button = Button::new(&mut self.reload_state, Text::new("Reload Resources"))
            .on_press(EditorMessage::ReloadResources);

        let save_button = Button::new(&mut self.save_state, Text::new("Save the tilemap"))
            .on_press(EditorMessage::SaveTilemap);

        let export_button = Button::new(&mut self.export_state, Text::new("Export"))
            .on_press(EditorMessage::ExportTilemap);

        Column::new()
            .push(reload_button)
            .push(save_button)
            .push(export_button)
            .push(title("Help"))
            .push(help("Press R to reload resources"))
            .push(help("Press S to save the tilemap"))
            .push(help("Press Space to export as images"))
    }
}
