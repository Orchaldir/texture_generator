use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::Tool;
use iced::mouse::Button;
use texture_generation::math::point::Point;
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
}
