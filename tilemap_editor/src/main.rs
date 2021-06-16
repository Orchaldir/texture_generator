#[macro_use]
extern crate log;
use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::preview::widget::Preview;
use crate::resources::ResourceInfo;
use crate::toolbar::Toolbar;
use iced::keyboard::KeyCode;
use iced::{image, Column, Element, Row, Sandbox, Settings};
use structopt::StructOpt;
use texture_generation::generation::io::{save_color_image, save_depth_image};
use texture_generation::utils::logging::init_logging;
use tool::tools::Tools;

mod data;
mod message;
mod preview;
mod resources;
mod tool;
mod toolbar;

pub fn main() -> iced::Result {
    init_logging();
    TilemapEditor::run(Settings::default())
}

struct TilemapEditor {
    data: EditorData,
    image: image::Handle,
    tools: Tools,
    toolbar: Toolbar,
}

impl TilemapEditor {
    fn export_tilemap(&self) -> bool {
        info!("Export the tilemap as color & depth images");
        let data = self
            .data
            .renderer
            .render(&self.data.tilemap, Some(&self.data.furniture_map));
        save_color_image(&data, "tilemap-color.png");
        save_depth_image(&data, "tilemap-depth.png");
        info!("Finished saving tilemap images");
        false
    }
}

impl Sandbox for TilemapEditor {
    type Message = EditorMessage;

    fn new() -> TilemapEditor {
        let data = EditorData::new(ResourceInfo::from_args());
        let image = data.render_preview();

        TilemapEditor {
            data,
            image,
            tools: Tools::new(),
            toolbar: Toolbar::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Tilemap Editor")
    }

    fn update(&mut self, message: Self::Message) {
        info!("Got message {:?}", message);

        let trigger_preview = match message {
            EditorMessage::ReloadResources | EditorMessage::PressedKey(KeyCode::R) => {
                self.data.reload_resources();
                true
            }
            EditorMessage::ExportTilemap | EditorMessage::PressedKey(KeyCode::Space) => {
                self.export_tilemap()
            }
            _ => self.tools.update(&mut self.data, message),
        };

        if trigger_preview {
            info!("Update triggered preview");
            self.image = self.data.render_preview();
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let toolbar = self
            .toolbar
            .view_toolbar(self.tools.get_tool_names(), self.tools.get_current_tool());
        let sidebar = self.tools.view_sidebar(&self.data);

        let main = Row::new()
            .push(Preview::new(self.image.clone()))
            .push(sidebar);
        Column::new().push(toolbar).push(main).into()
    }
}
