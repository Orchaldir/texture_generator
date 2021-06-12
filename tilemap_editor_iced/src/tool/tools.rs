use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::tile::TileTool;
use crate::tool::Tool;
use iced::Element;
use iced_native::{button, Align, Button, Row, Text};

pub struct Tools {
    tool: Box<dyn Tool>,
    button_state: button::State,
}

impl<'a> Tools {
    pub fn new() -> Self {
        Tools {
            tool: Box::new(TileTool::default()),
            button_state: button::State::new(),
        }
    }

    pub fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        self.tool.update(data, message)
    }

    pub fn view_toolbar(&'a mut self) -> Element<'a, EditorMessage> {
        Row::new()
            .padding(10)
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.button_state, Text::new(self.tool.get_name()))
                    .on_press(EditorMessage::Render),
            )
            .into()
    }
}
