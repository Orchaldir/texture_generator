use anyhow::{bail, Result};
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

#[derive(Clone, Debug, PartialEq)]
pub struct HandleStyle {
    distance_to_end: i32,
    offset: i32,
    horizontal_size: Size,
    vertical_size: Size,
    component: RenderingComponent,
}

impl HandleStyle {
    pub fn new(
        distance_to_end: u32,
        offset: u32,
        size: Size,
        component: RenderingComponent,
    ) -> Result<HandleStyle> {
        if size.width() == 0 {
            bail!("Argument 'size.width' needs to be greater than 0");
        } else if size.height() == 0 {
            bail!("Argument 'size.height' needs to be greater than 0");
        }

        Ok(HandleStyle {
            distance_to_end: distance_to_end as i32,
            offset: offset as i32,
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
        offset: i32,
        texture: &mut Texture,
    ) {
        let aabb = self.calculate_horizontal_aabb(node, edge, offset, true);
        self.component.render(texture, &data.transform(aabb));

        let aabb = self.calculate_horizontal_aabb(node, edge, offset, false);
        self.component.render(texture, &data.transform(aabb));
    }

    pub fn render_vertical(
        &self,
        data: &Data,
        node: Point,
        edge: (i32, u32),
        offset: i32,
        texture: &mut Texture,
    ) {
        let aabb = self.calculate_vertical_aabb(node, edge, offset, true);
        self.component.render(texture, &data.transform(aabb));

        let aabb = self.calculate_vertical_aabb(node, edge, offset, false);
        self.component.render(texture, &data.transform(aabb));
    }

    fn calculate_horizontal_aabb(
        &self,
        node: Point,
        edge: (i32, u32),
        offset: i32,
        is_front: bool,
    ) -> AABB {
        let (start, length) = edge;
        let end = node.x + start + length as i32;
        let handle_offset = if is_front {
            self.offset
        } else {
            -(self.offset + self.horizontal_size.height() as i32)
        };
        let start = Point::new(
            end - self.distance_to_end - self.horizontal_size.width() as i32,
            node.y + offset + handle_offset,
        );
        AABB::new(start, self.horizontal_size)
    }

    fn calculate_vertical_aabb(
        &self,
        node: Point,
        edge: (i32, u32),
        offset: i32,
        is_front: bool,
    ) -> AABB {
        let (start, length) = edge;
        let end = node.y + start + length as i32;
        let handle_offset = if is_front {
            self.offset
        } else {
            -(self.offset + self.vertical_size.width() as i32)
        };
        let start = Point::new(
            node.x + offset + handle_offset,
            end - self.distance_to_end - self.vertical_size.height() as i32,
        );
        AABB::new(start, self.vertical_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::math::color::{BLACK, GREEN};

    #[test]
    #[should_panic]
    fn test_new_with_0_width() {
        HandleStyle::new(0, 0, Size::new(0, 10), RenderingComponent::Mock).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_with_0_height() {
        HandleStyle::new(0, 0, Size::new(20, 0), RenderingComponent::Mock).unwrap();
    }

    #[test]
    fn test_render_horizontal() {
        let component = RenderingComponent::new_fill_area(GREEN, 4);
        let handle = HandleStyle::new(2, 1, Size::new(3, 2), component).unwrap();
        let mut texture = Texture::new(Size::new(11, 8), BLACK);

        handle.render_horizontal(
            &Data::for_texture(texture.get_aabb()),
            Point::new(1, 4),
            (1, 9),
            0,
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
        let handle = HandleStyle::new(2, 1, Size::new(3, 2), component).unwrap();
        let mut texture = Texture::new(Size::new(8, 11), BLACK);

        handle.render_vertical(
            &Data::for_texture(texture.get_aabb()),
            Point::new(4, 1),
            (1, 9),
            0,
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
