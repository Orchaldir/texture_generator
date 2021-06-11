#[macro_use]
extern crate log;
use crate::resources::ResourceInfo;
use iced::{image, Column, Element, Sandbox, Settings, Text};
use structopt::StructOpt;
use texture_generation::math::color::convert_bgra;

mod resources;

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {
    resource_info: ResourceInfo,
    image: image::Handle,
    image_viewer: image::viewer::State,
}

impl Sandbox for Hello {
    type Message = ();

    fn new() -> Hello {
        let resource_info: ResourceInfo = ResourceInfo::from_args();
        let (renderer, preview_renderer) = resource_info.create_renderers();
        let (tilemap, furniture_map) = resource_info.load_tilemap();

        let data = preview_renderer.render(&tilemap, Some(&furniture_map));
        let rbg = convert_bgra(data.get_color_data());
        let size = data.get_size();

        Hello {
            resource_info,
            image: image::Handle::from_pixels(size.width(), size.height(), rbg),
            image_viewer: image::viewer::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .spacing(20)
            .push(image::Viewer::new(
                &mut self.image_viewer,
                self.image.clone(),
            ))
            .push(Text::new("Tilemap"))
            .into()
    }
}
