use iced_graphics::{backend, Backend, Primitive};
use iced_native::{image, mouse, Rectangle, Size, Vector};

pub trait Renderer: iced_native::Renderer + Sized {
    fn draw(
        &mut self,
        bounds: Rectangle,
        image_size: Size,
        translation: Vector,
        handle: image::Handle,
        is_mouse_over: bool,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend + backend::Image,
{
    fn draw(
        &mut self,
        bounds: Rectangle,
        image_size: Size,
        translation: Vector,
        handle: image::Handle,
        _is_mouse_over: bool,
    ) -> Self::Output {
        (
            {
                Primitive::Clip {
                    bounds,
                    content: Box::new(Primitive::Translate {
                        translation,
                        content: Box::new(Primitive::Image {
                            handle,
                            bounds: Rectangle {
                                x: bounds.x,
                                y: bounds.y,
                                ..Rectangle::with_size(image_size)
                            },
                        }),
                    }),
                    offset: Vector::new(0, 0),
                }
            },
            { mouse::Interaction::Idle },
        )
    }
}
