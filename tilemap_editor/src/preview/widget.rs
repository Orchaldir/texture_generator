use crate::message::EditorMessage;
use iced_native::layout::{Limits, Node};
use iced_native::{
    event, image, mouse, Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle, Size,
    Vector, Widget,
};
use std::hash::Hash;

pub struct Preview {
    handle: image::Handle,
}

impl Preview {
    pub fn new(handle: image::Handle) -> Self {
        Preview { handle }
    }

    /// Returns the bounds of the underlying image, given the bounds of
    /// the [`Preview`]. Original aspect ratio will be respected.
    fn image_size<Renderer>(&self, renderer: &Renderer, bounds: Size) -> Size
    where
        Renderer: crate::preview::renderer::Renderer + image::Renderer,
    {
        let (width, height) = renderer.dimensions(&self.handle);
        let (width, height) = (width as f32, height as f32);

        let width_ratio = bounds.width / width;
        let height_ratio = bounds.height / height;

        let ratio = width_ratio.min(height_ratio);

        Size::new(width * ratio, height * ratio)
    }

    fn get_image_top_left(&self, bounds: Rectangle, image_size: Size) -> Vector {
        Vector::new(
            bounds.width / 2.0 - image_size.width / 2.0,
            bounds.height / 2.0 - image_size.height / 2.0,
        )
    }

    fn get_image_rectangle<Renderer>(&self, renderer: &Renderer, bounds: Rectangle) -> Rectangle
    where
        Renderer: crate::preview::renderer::Renderer + image::Renderer,
    {
        let image_size = self.image_size(renderer, bounds.size());
        let image_top_left = bounds.position() + self.get_image_top_left(bounds, image_size);
        Rectangle::new(image_top_left, image_size)
    }
}

impl<Renderer> Widget<EditorMessage, Renderer> for Preview
where
    Renderer: crate::preview::renderer::Renderer + image::Renderer,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Fill
    }

    fn layout(&self, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(limits.max())
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        let bounds = layout.bounds();
        let image_size = self.image_size(renderer, bounds.size());
        let image_top_left = self.get_image_top_left(bounds, image_size);

        let is_mouse_over = bounds.contains(cursor_position);

        crate::preview::renderer::Renderer::draw(
            renderer,
            bounds,
            image_size,
            image_top_left,
            self.handle.clone(),
            is_mouse_over,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.handle.hash(state);
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<EditorMessage>,
    ) -> event::Status {
        match event {
            Event::Keyboard(event) => match event {
                iced::keyboard::Event::KeyReleased { key_code, .. } => {
                    let image = self.get_image_rectangle(renderer, layout.bounds());

                    if image.contains(cursor_position) {
                        info!("Released key {:?}", key_code);
                        messages.push(EditorMessage::PressedKey(key_code))
                    }
                }
                _ => {}
            },
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(button) => {
                    let bounds = layout.bounds();
                    let image = self.get_image_rectangle(renderer, bounds);

                    if image.contains(cursor_position) {
                        let (width, height) = renderer.dimensions(&self.handle);
                        let position = cursor_position - image.position();
                        info!("Clicked at {:?} with {:?}", position, button);
                        messages.push(EditorMessage::ClickedButton {
                            x: (width as f32 * position.x / image.width) as u32,
                            y: (height as f32 * position.y / image.height) as u32,
                            button,
                        })
                    }
                }
                _ => {}
            },
            _ => {}
        }
        event::Status::Ignored
    }
}

impl<'a, Renderer> From<Preview> for Element<'a, EditorMessage, Renderer>
where
    Renderer: crate::preview::renderer::Renderer + image::Renderer,
{
    fn from(preview: Preview) -> Element<'a, EditorMessage, Renderer> {
        Element::new(preview)
    }
}
