use iced_native::layout::{Limits, Node};
use iced_native::{image, Element, Hasher, Layout, Length, Point, Rectangle, Size, Vector, Widget};
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
}

impl<Message, Renderer> Widget<Message, Renderer> for Preview
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

        let translation = {
            let image_top_left = Vector::new(
                bounds.width / 2.0 - image_size.width / 2.0,
                bounds.height / 2.0 - image_size.height / 2.0,
            );

            image_top_left
        };

        let is_mouse_over = bounds.contains(cursor_position);

        crate::preview::renderer::Renderer::draw(
            renderer,
            bounds,
            image_size,
            translation,
            self.handle.clone(),
            is_mouse_over,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.handle.hash(state);
    }
}

impl<'a, Message, Renderer> From<Preview> for Element<'a, Message, Renderer>
where
    Renderer: crate::preview::renderer::Renderer + image::Renderer,
    Message: 'a,
{
    fn from(preview: Preview) -> Element<'a, Message, Renderer> {
        Element::new(preview)
    }
}
