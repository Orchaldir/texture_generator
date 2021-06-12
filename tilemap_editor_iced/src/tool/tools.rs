use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::tile::TileTool;
use crate::tool::wall::WallTool;
use crate::tool::Tool;
use iced::Element;
use iced_native::{button, Align, Button, Row, Text};

pub struct Tools {
    tile_tool: TileTool,
    wall_tool: WallTool,
    tile_state: button::State,
    wall_state: button::State,
    current_tool: usize,
}

impl<'a> Tools {
    pub fn new() -> Self {
        Tools {
            tile_tool: TileTool::default(),
            wall_tool: WallTool::default(),
            tile_state: button::State::new(),
            wall_state: button::State::new(),
            current_tool: 0,
        }
    }

    fn get_tool(&mut self) -> &mut dyn Tool {
        match self.current_tool {
            1 => &mut self.wall_tool,
            _ => &mut self.tile_tool,
        }
    }

    pub fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeTool(id) => self.current_tool = id,
            _ => return self.get_tool().update(data, message),
        }

        return false;
    }

    pub fn view_toolbar(&'a mut self) -> Element<'a, EditorMessage> {
        Row::new()
            .padding(10)
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.tile_state, Text::new(self.tile_tool.get_name()))
                    .on_press(EditorMessage::ChangeTool(0)),
            )
            .push(
                Button::new(&mut self.wall_state, Text::new(self.wall_tool.get_name()))
                    .on_press(EditorMessage::ChangeTool(1)),
            )
            .into()
    }
}
