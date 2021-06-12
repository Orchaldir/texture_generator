use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::Tool;
use iced::mouse::Button;
use iced::{Column, Element, Text};
use texture_generation::math::point::Point;
use texture_generation::utils::resource::Resource;
use tilemap::tilemap::tile::Tile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TileTool {
    texture_id: usize,
}

impl Tool for TileTool {
    fn get_name(&self) -> &str {
        "Tile"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ClickedButton { x, y, button } => {
                let point = Point::new(x as i32, y as i32);

                if let Some(index) = data.selector.get_tile_index(&data.tilemap, point) {
                    let tile = match button {
                        Button::Left => Tile::Floor(self.texture_id),
                        Button::Right => Tile::Empty,
                        Button::Middle => Tile::Solid(self.texture_id),
                        _ => return false,
                    };

                    info!("Set tile {} to {:?}", index, tile);

                    data.tilemap.set_tile(index, tile);
                    return true;
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
            .get(self.texture_id)
            .get_name();
        Column::new()
            .max_width(800)
            .spacing(20)
            .push(Text::new(&format!("Texture: {}", name)))
            .into()
    }
}
