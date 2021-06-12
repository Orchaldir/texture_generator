#[macro_use]
extern crate log;
use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::preview::widget::Preview;
use crate::resources::ResourceInfo;
use crate::tool::tile::TileTool;
use crate::tool::Tool;
use iced::{image, Column, Element, Sandbox, Settings, Text};
use structopt::StructOpt;
use texture_generation::utils::logging::init_logging;

mod data;
mod message;
mod preview;
mod resources;
mod tool;

pub fn main() -> iced::Result {
    init_logging();
    Hello::run(Settings::default())
}

struct Hello {
    data: EditorData,
    image: image::Handle,
    tools: Vec<Box<dyn Tool>>,
    current_tool: usize,
}

impl Sandbox for Hello {
    type Message = EditorMessage;

    fn new() -> Hello {
        let data = EditorData::new(ResourceInfo::from_args());
        let image = data.render_preview();

        Hello {
            data,
            image,
            tools: vec![Box::new(TileTool::default())],
            current_tool: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Tilemap Editor")
    }

    fn update(&mut self, message: Self::Message) {
        info!("Got message {:?}", message);

        match message {
            EditorMessage::Render => {
                self.image = self.data.render_preview();
            }
            _ => {
                if let Some(tool) = self.tools.get_mut(self.current_tool) {
                    if tool.update(&mut self.data, message) {
                        self.image = self.data.render_preview();
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .spacing(20)
            .push(Preview::new(self.image.clone()))
            .push(Text::new("Tilemap"))
            .into()
    }
}
