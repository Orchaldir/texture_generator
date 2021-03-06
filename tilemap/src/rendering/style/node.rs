use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;
use texture_generation::utils::resource::Resource;

#[derive(Clone, Debug, PartialEq)]
pub struct NodeStyle {
    name: String,
    size: Size,
    half: i32,
    component: RenderingComponent,
}

impl NodeStyle {
    pub fn default_with_size(size: u32) -> NodeStyle {
        Self::new("default", size, RenderingComponent::default())
    }

    pub fn new<S: Into<String>>(name: S, size: u32, component: RenderingComponent) -> NodeStyle {
        NodeStyle {
            name: name.into(),
            size: Size::square(size),
            half: (size / 2) as i32,
            component,
        }
    }

    pub fn get_half(&self) -> i32 {
        self.half
    }

    pub fn render(&self, data: &Data, node: Point, texture: &mut Texture) {
        let start = node - self.half;
        let aabb = AABB::new(start, self.size);
        self.component.render(texture, &data.transform(aabb))
    }
}

impl Default for NodeStyle {
    fn default() -> Self {
        NodeStyle::default_with_size(1)
    }
}

impl Resource for NodeStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::data::texture::Texture;
    use texture_generation::math::color::{BLACK, RED};

    #[test]
    fn test_render_node() {
        let component = RenderingComponent::new_fill_area(RED, 9);
        let node_style = NodeStyle::new("node", 2, component);
        let mut texture = Texture::new(Size::new(6, 5), BLACK);

        node_style.render(
            &Data::for_texture(texture.get_aabb()),
            Point::new(3, 2),
            &mut texture,
        );

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK,   RED,   RED, BLACK, BLACK,
            BLACK, BLACK,   RED,   RED, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        ];

        assert_eq!(texture.get_color_data(), &result);

        #[rustfmt::skip]
        let depth = vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 9, 9, 0, 0,
            0, 0, 9, 9, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(texture.get_depth_data(), &depth);
    }
}
