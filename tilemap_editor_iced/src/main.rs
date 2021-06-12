#[macro_use]
extern crate log;
use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::preview::widget::Preview;
use crate::resources::ResourceInfo;
use iced::{image, Column, Element, Sandbox, Settings, Text};
use structopt::StructOpt;
use texture_generation::utils::logging::init_logging;

mod data;
mod message;
mod preview;
mod resources;

pub fn main() -> iced::Result {
    init_logging();
    Hello::run(Settings::default())
}

struct Hello {
    data: EditorData,
    image: image::Handle,
}

impl Sandbox for Hello {
    type Message = EditorMessage;

    fn new() -> Hello {
        let data = EditorData::new(ResourceInfo::from_args());
        let image = data.render_preview();

        Hello { data, image }
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
            _ => {}
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
