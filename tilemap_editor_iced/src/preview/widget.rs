use iced_native::layout::{Limits, Node};
use iced_native::{image, Element, Hasher, Layout, Length, Point, Rectangle, Size, Vector, Widget};
use std::hash::Hash;

pub struct Preview {
    width: Length,
    height: Length,
    handle: image::Handle,
}

impl Preview {
    pub fn new(handle: image::Handle) -> Self {
        Preview {
            width: Length::Shrink,
            height: Length::Shrink,
            handle,
        }
    }

    /// Sets the width of the [`Preview`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Preview`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Returns the bounds of the underlying image, given the bounds of
    /// the [`Preview`]. Original aspect ratio will be respected.
    fn image_size<Renderer>(&self, renderer: &Renderer, bounds: Size) -> Size
    where
        Renderer: crate::preview::renderer::Renderer + image::Renderer,
    {
        let (width, height) = renderer.dimensions(&self.handle);

        let (width, height) = {
            let dimensions = (width as f32, height as f32);

            let width_ratio = bounds.width / dimensions.0;
            let height_ratio = bounds.height / dimensions.1;

            let ratio = width_ratio.min(height_ratio);

            if ratio < 1.0 {
                (dimensions.0 * ratio, dimensions.1 * ratio)
            } else {
                (dimensions.0, dimensions.1)
            }
        };

        Size::new(width, height)
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Preview
where
    Renderer: crate::preview::renderer::Renderer + image::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let (width, height) = renderer.dimensions(&self.handle);

        let aspect_ratio = width as f32 / height as f32;

        let mut size = limits
            .width(self.width)
            .height(self.height)
            .resolve(Size::new(width as f32, height as f32));

        let viewport_aspect_ratio = size.width / size.height;

        if viewport_aspect_ratio > aspect_ratio {
            size.width = width as f32 * size.height / height as f32;
        } else {
            size.height = height as f32 * size.width / width as f32;
        }

        Node::new(size)
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
        self.width.hash(state);
        self.height.hash(state);

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
