use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, Tool};
use iced::mouse::Button;
use iced::{pick_list, Column, Element, Text};
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
                if let Some(id) = data.renderer.get_resources().furniture_styles.get_id(&name) {
                    info!(
                        "FurnitureTool: Change furniture style to '{}' with id {}",
                        &name, id
                    );
                    self.furniture_style_id = id;
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
                                data.furniture_map.remove_furniture(id)
                            }
                        };
                    }
                }
            }
            _ => {}
        }

        return false;
    }

    fn view_sidebar(&mut self, data: &EditorData) -> Element<'_, EditorMessage> {
        let pick_list = create_pick_list(
            &data.renderer.get_resources().furniture_styles,
            &mut self.furniture_style_state,
            self.furniture_style_id,
            EditorMessage::ChangeFurnitureStyle,
        );

        Column::new()
            .max_width(800)
            .spacing(20)
            .push(Text::new("Furniture Style"))
            .push(pick_list)
            .into()
    }
}
