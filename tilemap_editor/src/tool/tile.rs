use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, help, title, Tool};
use iced::mouse::Button;
use iced::{pick_list, Column, Text};
use texture_generation::math::point::Point;
use tilemap::tilemap::tile::Tile;

#[derive(Clone, Debug, Default)]
pub struct TileTool {
    texture_id: usize,
    texture_state: pick_list::State<String>,
}

impl Tool for TileTool {
    fn get_name(&self) -> &str {
        "Tile"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeTexture(name) => {
                if let Some(id) = data.renderer.get_resources().textures.get_id(&name) {
                    info!("TileTool: Change texture to '{}' with id {}", &name, id);
                    self.texture_id = id;
                }
            }
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

        false
    }

    fn view_sidebar(&mut self, data: &EditorData) -> Column<EditorMessage> {
        let pick_list = create_pick_list(
            &data.renderer.get_resources().textures,
            &mut self.texture_state,
            self.texture_id,
            EditorMessage::ChangeTexture,
        );

        Column::new()
            .push(Text::new("Tile Texture"))
            .push(pick_list)
            .push(title("Help"))
            .push(help("Click right to paint a floor tile"))
            .push(help(
                "Click middle button to paint a full tile, which blocks movement",
            ))
            .push(help("Click left to remove a tile"))
    }
}
