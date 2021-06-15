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
    Hello::run(Settings::default())
}

struct Hello {
    data: EditorData,
    image: image::Handle,
    tools: Tools,
    toolbar: Toolbar,
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
            toolbar: Toolbar::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Tilemap Editor")
    }

    fn update(&mut self, message: Self::Message) {
        info!("Got message {:?}", message);

        let trigger_preview = match message {
            EditorMessage::Render => true,
            EditorMessage::PressedKey(KeyCode::R) => {
                self.data.reload_resources();
                true
            }
            _ => self.tools.update(&mut self.data, message),
        };

        if trigger_preview {
            info!("Update triggered rendering");
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
