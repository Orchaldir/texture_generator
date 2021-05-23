use anyhow::{bail, Result};
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

#[derive(Clone, Debug, PartialEq)]
pub struct HandleStyle {
    on_both_sides: bool,
    horizontal_pos: Point,
    vertical_pos: Point,
    horizontal_size: Size,
    vertical_size: Size,
    component: RenderingComponent,
}

impl HandleStyle {
    pub fn new(
        on_both_sides: bool,
        pos: Point,
        size: Size,
        component: RenderingComponent,
    ) -> Result<HandleStyle> {
        if size.width() == 0 {
            bail!("Argument 'size.width' needs to be greater than 0");
        } else if size.height() == 0 {
            bail!("Argument 'size.height' needs to be greater than 0");
        }

        Ok(HandleStyle {
            on_both_sides,
            horizontal_pos: pos,
            vertical_pos: pos.flip(),
            horizontal_size: size,
            vertical_size: size.flip(),
            component,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;

    #[test]
    #[should_panic]
    fn test_new_with_0_width() {
        HandleStyle::new(
            true,
            Point::default(),
            Size::new(0, 10),
            RenderingComponent::Mock,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_with_0_height() {
        HandleStyle::new(
            true,
            Point::default(),
            Size::new(20, 0),
            RenderingComponent::Mock,
        )
        .unwrap();
    }
}
