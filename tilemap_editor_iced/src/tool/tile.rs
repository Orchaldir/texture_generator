use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::Tool;
use texture_generation::math::point::Point;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TileTool {
    texture_id: usize,
}

impl Tool for TileTool {
    fn get_name(&self) -> &str {
        "Tile"
    }

    fn update(&mut self, data: &EditorData, message: EditorMessage) {
        match message {
            EditorMessage::ClickedButton { x, y, button } => {
                let point = Point::new(x as i32, y as i32);

                if let Some(index) = data.selector.get_tile_index(&data.tilemap, point) {
                    info!("Update tile {}", index);
                }
            }
            _ => {}
        }
    }
}
