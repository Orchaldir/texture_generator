use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, help, title, Tool};
use iced::mouse::Button;
use iced::{pick_list, Column, Text};
use texture_generation::math::point::Point;
use tilemap::tilemap::border::Border;

#[derive(Clone, Debug, Default)]
pub struct DoorTool {
    wall_id: usize,
    door_id: usize,
    wall_state: pick_list::State<String>,
    door_state: pick_list::State<String>,
}

impl Tool for DoorTool {
    fn get_name(&self) -> &str {
        "Door"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeWallStyle(name) => {
                if let Some(id) = data.renderer.get_resources().wall_styles.get_id(&name) {
                    info!("DoorTool: Change wall style to '{}' with id {}", &name, id);
                    self.wall_id = id;
                }
            }
            EditorMessage::ChangeDoorStyle(name) => {
                if let Some(id) = data.renderer.get_resources().door_styles.get_id(&name) {
                    info!("DoorTool: Change door style to '{}' with id {}", &name, id);
                    self.door_id = id;
                }
            }
            EditorMessage::ClickedButton { x, y, button } => {
                let point = Point::new(x as i32, y as i32);

                if let Some(index) = data.selector.get_tile_index(&data.tilemap, point) {
                    if let Some(side) = data.selector.get_side(&data.tilemap, point, index) {
                        let old_border = data.tilemap.get_border(index, side);

                        let border = match button {
                            Button::Left => match old_border {
                                Border::Door { .. } => old_border.switch_is_front(),
                                _ => Border::new_door(self.wall_id, self.door_id, true),
                            },
                            _ => old_border.reduce(),
                        };

                        if old_border != border {
                            info!("Set {:?} border of tile {} to {:?}", side, index, border);

                            data.tilemap.set_border(index, side, border);
                            return true;
                        }

                        return false;
                    }
                }
            }
            _ => {}
        }

        return false;
    }

    fn view_sidebar(&mut self, data: &EditorData) -> Column<EditorMessage> {
        let wall_pick_list = create_pick_list(
            &data.renderer.get_resources().wall_styles,
            &mut self.wall_state,
            self.wall_id,
            EditorMessage::ChangeWallStyle,
        );
        let door_pick_list = create_pick_list(
            &data.renderer.get_resources().door_styles,
            &mut self.door_state,
            self.door_id,
            EditorMessage::ChangeDoorStyle,
        );

        Column::new()
            .push(Text::new("Wall Style"))
            .push(wall_pick_list)
            .push(Text::new("Door Style"))
            .push(door_pick_list)
            .push(title("Help"))
            .push(help("Click right to add"))
            .push(help("Click right on a door to change its side"))
            .push(help("Click left to remove"))
    }
}
