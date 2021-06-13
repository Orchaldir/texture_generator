use crate::message::EditorMessage;
use iced::button::Style;
use iced::Element;
use iced_native::{button, Align, Background, Button, Color, Row, Text};

pub struct Toolbar {
    button_states: Vec<button::State>,
}

impl<'a> Toolbar {
    pub fn new() -> Self {
        Toolbar {
            button_states: Vec::new(),
        }
    }

    pub fn view_toolbar(
        &'a mut self,
        tools: Vec<&str>,
        current_tool: usize,
    ) -> Element<'a, EditorMessage> {
        if self.button_states.len() != tools.len() {
            self.button_states = vec![button::State::new(); tools.len()];
        }

        let create_button = |state, text, id| {
            Button::new(state, Text::new(text))
                .on_press(EditorMessage::ChangeTool(id))
                .style(ButtonStyle(id == current_tool))
        };

        let mut toolbar = Row::new()
            .padding(10)
            .spacing(20)
            .align_items(Align::Center);

        let it = tools.iter().zip(self.button_states.iter_mut());

        for (i, (tool, state)) in it.enumerate() {
            toolbar = toolbar.push(create_button(state, *tool, i));
        }

        toolbar.into()
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
