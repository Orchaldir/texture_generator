use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, Tool};
use iced::mouse::Button;
use iced::{
    pick_list, slider, Column, Element, HorizontalAlignment, Length, PickList, Slider, Text,
};
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
    side: Side,
    side_state: pick_list::State<Side>,
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
            side: Side::Bottom,
            side_state: Default::default(),
        }
    }
}

impl Tool for FurnitureTool {
    fn get_name(&self) -> &str {
        "Furniture"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeWidth(width) => self.width = width,
            EditorMessage::ChangeHeight(height) => self.height = height,
            EditorMessage::ChangeSide(side) => self.side = side,
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
                    self.style_id = style_id;
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
                        let start = data.furniture_map.get_size().to_point(index);
                        let size = Size::new(self.width, self.height);
                        data.furniture_map.add(Furniture::new(
                            self.style_id,
                            start,
                            size,
                            self.side,
                        ));
                        return true;
                    }
                }
            }
            _ => {}
        }

        return false;
    }

    fn view_sidebar(&mut self, data: &EditorData) -> Element<'_, EditorMessage> {
        if let Some(id) = self.selected_id {
            let style_pick_list = create_pick_list(
                &data.renderer.get_resources().furniture_styles,
                &mut self.style_state,
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
                .push(style_pick_list)
                .into();
        }

        let style_pick_list = create_pick_list(
            &data.renderer.get_resources().furniture_styles,
            &mut self.style_state,
            self.style_id,
            EditorMessage::ChangeFurnitureStyle,
        );

        let width_slider = Slider::new(
            &mut self.width_state,
            1..=self.max_size,
            self.width,
            EditorMessage::ChangeWidth,
        );
        let height_slider = Slider::new(
            &mut self.height_state,
            1..=self.max_size,
            self.height,
            EditorMessage::ChangeHeight,
        );

        let options: Vec<Side> = Side::iterator().map(|s| s.clone()).collect();
        let side_pick_list = PickList::new(
            &mut self.side_state,
            options,
            Some(self.side),
            EditorMessage::ChangeSide,
        );

        Column::new()
            .max_width(250)
            .spacing(20)
            .push(Text::new("Furniture Style"))
            .push(style_pick_list)
            .push(Text::new(format!("Width: {} cells", self.width)))
            .push(width_slider)
            .push(Text::new(format!("Height: {} cells", self.height)))
            .push(height_slider)
            .push(Text::new("Front Side"))
            .push(side_pick_list)
            .into()
    }
}
