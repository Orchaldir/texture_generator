use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, Tool};
use iced::mouse::Button;
use iced::{pick_list, Column, Element, HorizontalAlignment, Length, Text};
use texture_generation::math::point::Point;

#[derive(Clone, Debug, Default)]
pub struct FurnitureTool {
    selected_id: Option<usize>,
    furniture_style_id: usize,
    furniture_style_state: pick_list::State<String>,
}

impl Tool for FurnitureTool {
    fn get_name(&self) -> &str {
        "Furniture"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeFurnitureStyle(name) => {
                if let Some(style_id) = data.renderer.get_resources().furniture_styles.get_id(&name)
                {
                    if let Some(id) = self.selected_id {
                        if let Some(furniture) = data.furniture_map.get_furniture_mut(id) {
                            info!(
                                "FurnitureTool: Change furniture {}'s style to '{}' with id {}",
                                id, &name, style_id
                            );
                            furniture.style_id = style_id;
                            return true;
                        }
                    }
                    info!(
                        "FurnitureTool: Change furniture style to '{}' with id {}",
                        &name, style_id
                    );
                    self.furniture_style_id = style_id;
                }
            }
            EditorMessage::ClickedButton { x, y, button } => {
                let point = Point::new(x as i32, y as i32);

                if let Some(index) = data
                    .selector
                    .get_furniture_tile_index(&data.furniture_map, point)
                {
                    if let Some(id) = data.furniture_map.get_id_at(index) {
                        return match button {
                            Button::Left => {
                                self.selected_id = Some(id);
                                false
                            }
                            _ => {
                                info!("FurnitureTool: Remove furniture with id {}", id);
                                self.selected_id = None;
                                data.furniture_map.remove_furniture(id)
                            }
                        };
                    } else {
                        self.selected_id = None;
                    }
                }
            }
            _ => {}
        }

        return false;
    }

    fn view_sidebar(&mut self, data: &EditorData) -> Element<'_, EditorMessage> {
        if let Some(id) = self.selected_id {
            let pick_list = create_pick_list(
                &data.renderer.get_resources().furniture_styles,
                &mut self.furniture_style_state,
                id,
                EditorMessage::ChangeFurnitureStyle,
            );

            return Column::new()
                .max_width(250)
                .spacing(20)
                .push(
                    Text::new("Furniture")
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .push(Text::new(format!("Id = {}", id)))
                .push(Text::new("Style"))
                .push(pick_list)
                .into();
        }

        let pick_list = create_pick_list(
            &data.renderer.get_resources().furniture_styles,
            &mut self.furniture_style_state,
            self.furniture_style_id,
            EditorMessage::ChangeFurnitureStyle,
        );

        Column::new()
            .max_width(250)
            .spacing(20)
            .push(Text::new("Furniture Style"))
            .push(pick_list)
            .into()
    }
}
