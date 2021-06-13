use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, Tool};
use iced::mouse::Button;
use iced::{pick_list, Column, Text};
use texture_generation::math::point::Point;
use tilemap::tilemap::border::Border;

#[derive(Clone, Debug, Default)]
pub struct WallTool {
    wall_id: usize,
    pick_list_state: pick_list::State<String>,
}

impl Tool for WallTool {
    fn get_name(&self) -> &str {
        "Wall"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeWallStyle(name) => {
                if let Some(id) = data.renderer.get_resources().wall_styles.get_id(&name) {
                    info!("WallTool: Change wall style to '{}' with id {}", &name, id);
                    self.wall_id = id;
                }
            }
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

    fn view_sidebar(&mut self, data: &EditorData) -> Column<EditorMessage> {
        let resource_manager = &data.renderer.get_resources().wall_styles;
        let pick_list = create_pick_list(
            resource_manager,
            &mut self.pick_list_state,
            self.wall_id,
            EditorMessage::ChangeWallStyle,
        );
        Column::new().push(Text::new("Wall Style")).push(pick_list)
    }
}
