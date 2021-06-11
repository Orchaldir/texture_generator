use iced::{image, Column, Element, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {
    image: image::Handle,
    image_viewer: image::viewer::State,
}

impl Sandbox for Hello {
    type Message = ();

    fn new() -> Hello {
        Hello {
            image: image::Handle::from("tilemap-color.png"),
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
