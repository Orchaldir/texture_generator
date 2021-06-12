#[macro_use]
extern crate log;
use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::preview::widget::Preview;
use crate::resources::ResourceInfo;
use iced::{image, Column, Element, Sandbox, Settings};
use structopt::StructOpt;
use texture_generation::utils::logging::init_logging;
use tool::tools::Tools;

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
    tools: Tools,
}

impl Sandbox for Hello {
    type Message = EditorMessage;

    fn new() -> Hello {
        let data = EditorData::new(ResourceInfo::from_args());
        let image = data.render_preview();

        Hello {
            data,
            image,
            tools: Tools::new(),
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
                if self.tools.update(&mut self.data, message) {
                    self.image = self.data.render_preview();
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let toolbar = self.tools.view_toolbar();
        Column::new()
            .push(toolbar)
            .push(Preview::new(self.image.clone()))
            .into()
    }
}
