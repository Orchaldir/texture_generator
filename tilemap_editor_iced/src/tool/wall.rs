use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::Tool;
use iced::mouse::Button;
use iced::{Column, Element, Text};
use texture_generation::math::point::Point;
use texture_generation::utils::resource::Resource;
use tilemap::tilemap::border::Border;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct WallTool {
    wall_id: usize,
}

impl Tool for WallTool {
    fn get_name(&self) -> &str {
        "Wall"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ClickedButton { x, y, button } => {
                let point = Point::new(x as i32, y as i32);

                if let Some(index) = data.selector.get_tile_index(&data.tilemap, point) {
                    if let Some(side) = data.selector.get_side(&data.tilemap, point, index) {
                        let border = match button {
                            Button::Left => Border::Wall(self.wall_id),
                            _ => Border::Empty,
                        };

                        info!("Set {:?} border of tile {} to {:?}", side, index, border);

                        data.tilemap.set_border(index, side, border);
                        return true;
                    }
                }
            }
            _ => {}
        }

        return false;
    }

    fn view_sidebar(&self, data: &EditorData) -> Element<'_, EditorMessage> {
        let name = data
            .renderer
            .get_resources()
            .textures
            .get(self.wall_id)
            .get_name();
        Column::new()
            .max_width(800)
            .spacing(20)
            .push(Text::new(&format!("Wall-Style: {}", name)))
            .into()
    }
}
