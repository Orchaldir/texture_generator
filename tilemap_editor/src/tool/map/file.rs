use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{help, title, Tool};
use iced::{button, Button, Column, Text};

#[derive(Clone, Debug, Default)]
pub struct FileTool {
    reload_resources_state: button::State,
    render_state: button::State,
}

impl Tool for FileTool {
    fn get_name(&self) -> &str {
        "File"
    }

    fn update(&mut self, _data: &mut EditorData, _message: EditorMessage) -> bool {
        false
    }

    fn view_sidebar(&mut self, _data: &EditorData) -> Column<EditorMessage> {
        let reload_resources_button = Button::new(
            &mut self.reload_resources_state,
            Text::new("Reload Resources"),
        )
        .on_press(EditorMessage::ReloadResources);
        let render_button = Button::new(&mut self.render_state, Text::new("Export"))
            .on_press(EditorMessage::ExportTilemap);

        Column::new()
            .push(reload_resources_button)
            .push(render_button)
            .push(title("Help"))
            .push(help("Press R to reload resources"))
            .push(help("Press Space to export as images"))
    }
}
