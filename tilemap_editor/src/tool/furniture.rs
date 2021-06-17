use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, help, title, Tool};
use iced::keyboard::KeyCode;
use iced::mouse::Button;
use iced::{pick_list, slider, Column, PickList, Slider, Text};
use std::fmt::Debug;
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::side::Side;
use texture_generation::math::size::Size;
use tilemap::tilemap::furniture::Furniture;

#[derive(Clone, Debug)]
pub struct FurnitureTool {
    selected_id: Option<usize>,
    style_id: usize,
    style_state: pick_list::State<String>,
    width: u32,
    width_state: slider::State,
    height: u32,
    height_state: slider::State,
    max_size: u32,
    front: Side,
    front_state: pick_list::State<Side>,
}

impl FurnitureTool {
    pub fn new(width: u32, height: u32, max_size: u32) -> Self {
        FurnitureTool {
            selected_id: None,
            style_id: 0,
            style_state: Default::default(),
            width,
            width_state: Default::default(),
            height,
            height_state: Default::default(),
            max_size,
            front: Side::Bottom,
            front_state: Default::default(),
        }
    }

    fn update_aabb<T: Debug>(
        &mut self,
        data: &mut EditorData,
        id: usize,
        description: &str,
        value: T,
        f: fn(&AABB, T) -> AABB,
    ) -> bool {
        if let Some(old_furniture) = data.furniture_map.get_furniture(id) {
            info!(
                "FurnitureTool: Update furniture {}'s {} with {:?}",
                id, description, value
            );
            let furniture = Furniture {
                aabb: f(&old_furniture.aabb, value),
                ..*old_furniture
            };
            return data.furniture_map.update_furniture(id, furniture);
        }

        false
    }

    fn update_width(&mut self, data: &mut EditorData, width: u32) -> bool {
        if let Some(id) = self.selected_id {
            return self.update_aabb(data, id, "width", width, |old, w| {
                let start = old.start();
                let size = Size::new(w, old.size().height());
                AABB::new(start, size)
            });
        }

        self.width = width;
        false
    }

    fn update_height(&mut self, data: &mut EditorData, height: u32) -> bool {
        if let Some(id) = self.selected_id {
            return self.update_aabb(data, id, "height", height, |old, h| {
                let start = old.start();
                let size = Size::new(old.size().width(), h);
                AABB::new(start, size)
            });
        }

        self.height = height;
        false
    }

    fn move_furniture(&mut self, data: &mut EditorData, x: i32, y: i32) -> bool {
        if let Some(id) = self.selected_id {
            let delta = Point::new(x, y);
            return self.update_aabb(data, id, "position", delta, |old, delta| {
                AABB::new(old.start() + delta, old.size())
            });
        }
        false
    }

    fn update_front(&mut self, data: &mut EditorData, front: Side) -> bool {
        if let Some(id) = self.selected_id {
            if let Some(old_furniture) = data.furniture_map.get_furniture(id) {
                info!(
                    "FurnitureTool: Change furniture {}'s front to {}",
                    id, front
                );
                let furniture = Furniture {
                    front_side: front,
                    ..*old_furniture
                };
                return data.furniture_map.update_furniture(id, furniture);
            }
        }

        self.front = front;
        false
    }

    fn update_style(&mut self, data: &mut EditorData, style: String) -> bool {
        if let Some(style_id) = data
            .renderer
            .get_resources()
            .furniture_styles
            .get_id(&style)
        {
            if let Some(id) = self.selected_id {
                if let Some(old_furniture) = data.furniture_map.get_furniture(id) {
                    info!(
                        "FurnitureTool: Change furniture {}'s style to '{}' with id {}",
                        id, &style, style_id
                    );
                    let furniture = Furniture {
                        style_id,
                        ..*old_furniture
                    };
                    return data.furniture_map.update_furniture(id, furniture);
                }
            }
            info!(
                "FurnitureTool: Change furniture style to '{}' with id {}",
                &style, style_id
            );
            self.style_id = style_id;
        }

        false
    }
}

impl Tool for FurnitureTool {
    fn get_name(&self) -> &str {
        "Furniture"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeWidth(width) => self.update_width(data, width),
            EditorMessage::ChangeHeight(height) => self.update_height(data, height),
            EditorMessage::ChangeSide(front) => self.update_front(data, front),
            EditorMessage::ChangeFurnitureStyle(name) => self.update_style(data, name),
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
                        let furniture = Furniture::new(
                            self.style_id,
                            data.furniture_map.get_size().to_point(index),
                            Size::new(self.width, self.height),
                            self.front,
                        )
                        .unwrap();

                        if let Some(id) = data.furniture_map.add(furniture) {
                            info!("FurnitureTool: Add furniture with id {}", id);
                            self.selected_id = Some(id);
                            return true;
                        } else {
                            warn!("FurnitureTool: Failed to add furniture, because its partly outside the map!");
                        }
                    }
                }
                false
            }
            EditorMessage::PressedKey(KeyCode::Up) => self.move_furniture(data, 0, -1),
            EditorMessage::PressedKey(KeyCode::Left) => self.move_furniture(data, -1, 0),
            EditorMessage::PressedKey(KeyCode::Down) => self.move_furniture(data, 0, 1),
            EditorMessage::PressedKey(KeyCode::Right) => self.move_furniture(data, 1, 0),
            _ => false,
        }
    }

    fn view_sidebar(&mut self, data: &EditorData) -> Column<EditorMessage> {
        let (style_id, width, height, front) = if let Some(furniture_id) = self.selected_id {
            if let Some(furniture) = data.furniture_map.get_furniture(furniture_id) {
                let size = furniture.aabb.size();
                (
                    furniture.style_id,
                    size.width(),
                    size.height(),
                    furniture.front_side,
                )
            } else {
                return Column::new().push(Text::new("Invalid furniture selected!"));
            }
        } else {
            (self.style_id, self.width, self.height, self.front)
        };

        let style_pick_list = create_pick_list(
            &data.renderer.get_resources().furniture_styles,
            &mut self.style_state,
            style_id,
            EditorMessage::ChangeFurnitureStyle,
        );

        let width_slider = Slider::new(
            &mut self.width_state,
            1..=self.max_size,
            width,
            EditorMessage::ChangeWidth,
        );
        let height_slider = Slider::new(
            &mut self.height_state,
            1..=self.max_size,
            height,
            EditorMessage::ChangeHeight,
        );

        let options: Vec<Side> = Side::iterator().copied().collect();
        let side_pick_list = PickList::new(
            &mut self.front_state,
            options,
            Some(front),
            EditorMessage::ChangeSide,
        );

        let mut column = Column::new().push(title("Control"));

        if let Some(id) = self.selected_id {
            column = column.push(Text::new(format!("Id = {}", id)));
        }

        column
            .push(Text::new("Style"))
            .push(style_pick_list)
            .push(Text::new(format!("Width: {} cells", width)))
            .push(width_slider)
            .push(Text::new(format!("Height: {} cells", height)))
            .push(height_slider)
            .push(Text::new("Front"))
            .push(side_pick_list)
            .push(title("Help"))
            .push(help("Click right to add or select"))
            .push(help("Click left to delete"))
            .push(help("Arrow keys to move selected furniture"))
    }
}
