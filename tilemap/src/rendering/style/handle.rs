use anyhow::{bail, Result};
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

#[derive(Clone, Debug, PartialEq)]
pub enum HandlePosition {
    Centered,
    DistanceToEnd(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct HandleStyle {
    position: HandlePosition,
    offset: i32,
    both_sides: bool,
    horizontal_size: Size,
    vertical_size: Size,
    component: RenderingComponent,
}

impl HandleStyle {
    pub fn new(
        position: HandlePosition,
        offset: u32,
        both_sides: bool,
        size: Size,
        component: RenderingComponent,
    ) -> Result<HandleStyle> {
        if size.width() == 0 {
            bail!("Argument 'size.width' needs to be greater than 0");
        } else if size.height() == 0 {
            bail!("Argument 'size.height' needs to be greater than 0");
        }

        Ok(HandleStyle {
            position,
            offset: offset as i32,
            both_sides,
            horizontal_size: size,
            vertical_size: size.flip(),
            component,
        })
    }

    pub fn render_horizontal(
        &self,
        data: &Data,
        node: Point,
        edge: (i32, u32),
        is_front: bool,
        texture: &mut Texture,
    ) {
        if is_front || self.both_sides {
            let aabb = self.calculate_horizontal_aabb(node, edge, true);
            self.component.render(texture, &data.transform(aabb));
        }

        if !is_front || self.both_sides {
            let aabb = self.calculate_horizontal_aabb(node, edge, false);
            self.component.render(texture, &data.transform(aabb));
        }
    }

    pub fn render_vertical(
        &self,
        data: &Data,
        node: Point,
        edge: (i32, u32),
        is_front: bool,
        texture: &mut Texture,
    ) {
        if is_front || self.both_sides {
            let aabb = self.calculate_vertical_aabb(node, edge, true);
            self.component.render(texture, &data.transform(aabb));
        }

        if !is_front || self.both_sides {
            let aabb = self.calculate_vertical_aabb(node, edge, false);
            self.component.render(texture, &data.transform(aabb));
        }
    }

    fn calculate_horizontal_aabb(&self, node: Point, edge: (i32, u32), is_front: bool) -> AABB {
        let (start, length) = edge;
        let handle_offset = if is_front {
            self.offset
        } else {
            -(self.offset + self.horizontal_size.height() as i32)
        };
        let handle_length = self.horizontal_size.width() as i32;
        let start_x = match self.position {
            HandlePosition::Centered => node.x + start + (length as i32 - handle_length) / 2,
            HandlePosition::DistanceToEnd(distance) => {
                let end = node.x + start + length as i32;
                end - distance - handle_length
            }
        };
        let start = Point::new(start_x, node.y + handle_offset);
        AABB::new(start, self.horizontal_size)
    }

    fn calculate_vertical_aabb(&self, node: Point, edge: (i32, u32), is_front: bool) -> AABB {
        let (start, length) = edge;
        let handle_offset = if is_front {
            self.offset
        } else {
            -(self.offset + self.vertical_size.width() as i32)
        };
        let handle_length = self.vertical_size.height() as i32;
        let start_y = match self.position {
            HandlePosition::Centered => node.y + start + (length as i32 - handle_length) / 2,
            HandlePosition::DistanceToEnd(distance) => {
                let end = node.y + start + length as i32;
                end - distance - handle_length
            }
        };
        let start = Point::new(node.x + handle_offset, start_y);
        AABB::new(start, self.vertical_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::math::color::{BLACK, GREEN};
    use HandlePosition::*;

    #[test]
    #[should_panic]
    fn test_new_with_0_width() {
        HandleStyle::new(
            Centered,
            0,
            true,
            Size::new(0, 10),
            RenderingComponent::Mock,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_with_0_height() {
        HandleStyle::new(
            Centered,
            0,
            false,
            Size::new(20, 0),
            RenderingComponent::Mock,
        )
        .unwrap();
    }

    #[test]
    fn test_render_horizontal() {
        let component = RenderingComponent::new_fill_area(GREEN, 4);
        let position = DistanceToEnd(2);
        let handle = HandleStyle::new(position, 1, true, Size::new(3, 2), component).unwrap();
        let mut texture = Texture::new(Size::new(11, 8), BLACK);

        handle.render_horizontal(
            &Data::for_texture(texture.get_aabb()),
            Point::new(1, 4),
            (1, 9),
            true,
            &mut texture,
        );

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, GREEN, GREEN, GREEN, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        ];

        assert_eq!(texture.get_color_data(), &result);
    }

    #[test]
    fn test_render_vertical() {
        let component = RenderingComponent::new_fill_area(GREEN, 4);
        let position = DistanceToEnd(2);
        let handle = HandleStyle::new(position, 1, true, Size::new(3, 2), component).unwrap();
        let mut texture = Texture::new(Size::new(8, 11), BLACK);

        handle.render_vertical(
            &Data::for_texture(texture.get_aabb()),
            Point::new(4, 1),
            (1, 9),
            true,
            &mut texture,
        );

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, GREEN, GREEN, BLACK, BLACK, GREEN, GREEN, BLACK,
            BLACK, GREEN, GREEN, BLACK, BLACK, GREEN, GREEN, BLACK,
            BLACK, GREEN, GREEN, BLACK, BLACK, GREEN, GREEN, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        ];

        assert_eq!(texture.get_color_data(), &result);
    }
}
