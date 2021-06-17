use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::{create_pick_list, Tool};
use iced::{button, pick_list, slider, Button, Column, Slider, Text};
use texture_generation::math::size::Size;
use tilemap::tilemap::furniture::map2d::FurnitureMap2d;
use tilemap::tilemap::tile::Tile;
use tilemap::tilemap::tilemap2d::Tilemap2d;

#[derive(Clone, Debug)]
pub struct NewMapTool {
    max_size: u32,
    width: u32,
    width_state: slider::State,
    height: u32,
    height_state: slider::State,
    texture_id: usize,
    texture_state: pick_list::State<String>,
    button_state: button::State,
}

impl NewMapTool {
    pub fn new(width: u32, height: u32, max_size: u32) -> Self {
        NewMapTool {
            max_size,
            width,
            width_state: Default::default(),
            height,
            height_state: Default::default(),
            texture_id: 0,
            texture_state: Default::default(),
            button_state: Default::default(),
        }
    }
}

impl Tool for NewMapTool {
    fn get_name(&self) -> &str {
        "New"
    }

    fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeWidth(width) => self.width = width,
            EditorMessage::ChangeHeight(height) => self.height = height,
            EditorMessage::ChangeTexture(name) => {
                if let Some(id) = data.renderer.get_resources().textures.get_id(&name) {
                    info!("NewTool: Change texture to '{}' with id {}", &name, id);
                    self.texture_id = id;
                }
            }
            EditorMessage::NewMap => {
                let size = Size::new(self.width, self.height);
                let default = Tile::Floor(self.texture_id);
                data.tilemap = Tilemap2d::default(size, default);
                data.furniture_map = FurnitureMap2d::empty(size);
                return true;
            }
            _ => {}
        }

        false
    }

    fn view_sidebar(&mut self, data: &EditorData) -> Column<EditorMessage> {
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
        let texture_pick_list = create_pick_list(
            &data.renderer.get_resources().textures,
            &mut self.texture_state,
            self.texture_id,
            EditorMessage::ChangeTexture,
        );
        let new_button = Button::new(&mut self.button_state, Text::new("Create new Map"))
            .on_press(EditorMessage::NewMap);

        Column::new()
            .push(Text::new(format!("Map Width: {} tiles", self.width)))
            .push(width_slider)
            .push(Text::new(format!("Map Height: {} tiles", self.height)))
            .push(height_slider)
            .push(Text::new("Default Texture"))
            .push(texture_pick_list)
            .push(new_button)
    }
}
