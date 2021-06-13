use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, Tool};
use iced::mouse::Button;
use iced::{pick_list, slider, Column, HorizontalAlignment, Length, PickList, Slider, Text};
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;
use tilemap::tilemap::furniture::Furniture;
use tilemap::tilemap::Side;

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

    fn update_width(&mut self, data: &mut EditorData, width: u32) -> bool {
        if let Some(id) = self.selected_id {
            if let Some(furniture) = data.furniture_map.get_furniture_mut(id) {
                info!(
                    "FurnitureTool: Change furniture {}'s width to {}",
                    id, width
                );
                let start = furniture.aabb.start();
                let size = Size::new(width, furniture.aabb.size().height());
                furniture.aabb = AABB::new(start, size);
                return true;
            }
        }

        self.width = width;
        return false;
    }

    fn update_height(&mut self, data: &mut EditorData, height: u32) -> bool {
        if let Some(id) = self.selected_id {
            if let Some(furniture) = data.furniture_map.get_furniture_mut(id) {
                info!(
                    "FurnitureTool: Change furniture {}'s height to {}",
                    id, height
                );
                let start = furniture.aabb.start();
                let size = Size::new(furniture.aabb.size().width(), height);
                furniture.aabb = AABB::new(start, size);
                return true;
            }
        }

        self.height = height;
        return false;
    }

    fn update_front(&mut self, data: &mut EditorData, front: Side) -> bool {
        if let Some(id) = self.selected_id {
            if let Some(furniture) = data.furniture_map.get_furniture_mut(id) {
                info!(
                    "FurnitureTool: Change furniture {}'s front to {}",
                    id, front
                );
                furniture.front_side = front;
                return true;
            }
        }

        self.front = front;
        return false;
    }

    fn update_style(&mut self, data: &mut EditorData, style: String) -> bool {
        if let Some(style_id) = data
            .renderer
            .get_resources()
            .furniture_styles
            .get_id(&style)
        {
            if let Some(id) = self.selected_id {
                if let Some(furniture) = data.furniture_map.get_furniture_mut(id) {
                    info!(
                        "FurnitureTool: Change furniture {}'s style to '{}' with id {}",
                        id, &style, style_id
                    );
                    furniture.style_id = style_id;
                    return true;
                }
            }
            info!(
                "FurnitureTool: Change furniture style to '{}' with id {}",
                &style, style_id
            );
            self.style_id = style_id;
        }

        return false;
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
                        );

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

        let options: Vec<Side> = Side::iterator().map(|s| s.clone()).collect();
        let side_pick_list = PickList::new(
            &mut self.front_state,
            options,
            Some(front),
            EditorMessage::ChangeSide,
        );

        let mut column = Column::new().push(
            Text::new("Furniture")
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
        );

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
    }
}
