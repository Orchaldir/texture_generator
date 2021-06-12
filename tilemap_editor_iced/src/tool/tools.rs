use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::tile::TileTool;
use crate::tool::wall::WallTool;
use crate::tool::Tool;
use iced::button::Style;
use iced::Element;
use iced_native::{button, Align, Background, Button, Color, Row, Text};

pub struct Tools {
    tile_tool: TileTool,
    wall_tool: WallTool,
    tile_state: button::State,
    wall_state: button::State,
    current_tool: usize,
}

impl Tools {
    pub fn new() -> Self {
        Tools {
            tile_tool: TileTool::default(),
            wall_tool: WallTool::default(),
            tile_state: button::State::new(),
            wall_state: button::State::new(),
            current_tool: 0,
        }
    }

    fn get_tool(&self) -> &dyn Tool {
        match self.current_tool {
            1 => &self.wall_tool,
            _ => &self.tile_tool,
        }
    }

    fn get_tool_mut(&mut self) -> &mut dyn Tool {
        match self.current_tool {
            1 => &mut self.wall_tool,
            _ => &mut self.tile_tool,
        }
    }

    pub fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeTool(id) => self.current_tool = id,
            _ => return self.get_tool_mut().update(data, message),
        }

        return false;
    }

    pub fn view_toolbar(&mut self) -> Element<EditorMessage> {
        let current_id = self.current_tool;
        let create_button = |state, tool: &dyn Tool, id| {
            Button::new(state, Text::new(tool.get_name()))
                .on_press(EditorMessage::ChangeTool(id))
                .style(ButtonStyle(id == current_id))
        };

        Row::new()
            .padding(10)
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new("Tools:"))
            .push(create_button(&mut self.tile_state, &self.tile_tool, 0))
            .push(create_button(&mut self.wall_state, &self.wall_tool, 1))
            .into()
    }

    pub fn view_sidebar(&self, data: &EditorData) -> Element<EditorMessage> {
        self.get_tool().view_sidebar(data)
    }
}

struct ButtonStyle(bool);

impl iced::button::StyleSheet for ButtonStyle {
    fn active(&self) -> Style {
        if self.0 {
            iced::button::Style {
                background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
                border_radius: 10.0,
                text_color: Color::WHITE,
                ..iced::button::Style::default()
            }
        } else {
            iced::button::Style::default()
        }
    }
}
