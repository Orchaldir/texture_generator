use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, Tool};
use iced::mouse::Button;
use iced::{pick_list, Column, Element, Text};
use texture_generation::math::point::Point;
use tilemap::tilemap::border::Border;

#[derive(Clone, Debug, Default)]
pub struct WindowTool {
    wall_id: usize,
    window_id: usize,
    wall_state: pick_list::State<String>,
    window_state: pick_list::State<String>,
}

impl Tool for WindowTool {
    fn get_name(&self) -> &str {
        "Window"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeWallStyle(name) => {
                if let Some(id) = data.renderer.get_resources().wall_styles.get_id(&name) {
                    info!(
                        "WindowTool: Change wall style to '{}' with id {}",
                        &name, id
                    );
                    self.wall_id = id;
                }
            }
            EditorMessage::ChangeWindowStyle(name) => {
                if let Some(id) = data.renderer.get_resources().window_styles.get_id(&name) {
                    info!(
                        "WindowTool: Change window style to '{}' with id {}",
                        &name, id
                    );
                    self.window_id = id;
                }
            }
            EditorMessage::ClickedButton { x, y, button } => {
                let point = Point::new(x as i32, y as i32);

                if let Some(index) = data.selector.get_tile_index(&data.tilemap, point) {
                    if let Some(side) = data.selector.get_side(&data.tilemap, point, index) {
                        let old_border = data.tilemap.get_border(index, side);

                        let border = match button {
                            Button::Left => Border::new_window(self.wall_id, self.window_id),
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

    fn view_sidebar(&mut self, data: &EditorData) -> Element<'_, EditorMessage> {
        let wall_pick_list = create_pick_list(
            &data.renderer.get_resources().wall_styles,
            &mut self.wall_state,
            self.wall_id,
            EditorMessage::ChangeWallStyle,
        );
        let window_pick_list = create_pick_list(
            &data.renderer.get_resources().window_styles,
            &mut self.window_state,
            self.window_id,
            EditorMessage::ChangeWindowStyle,
        );
        Column::new()
            .max_width(800)
            .spacing(20)
            .push(Text::new("Wall Style"))
            .push(wall_pick_list)
            .push(Text::new("Window Style"))
            .push(window_pick_list)
            .into()
    }
}
