use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;
use texture_generation::math::size::Size;

#[derive(Clone, Debug, PartialEq)]
pub struct NodeStyle {
    size: Size,
    half: i32,
    component: RenderingComponent,
}

impl NodeStyle {
    pub fn default_with_size(size: u32) -> NodeStyle {
        Self::new(size, RenderingComponent::default())
    }

    pub fn new(size: u32, component: RenderingComponent) -> NodeStyle {
        NodeStyle {
            size: Size::square(size),
            half: (size / 2) as i32,
            component,
        }
    }

    pub fn get_half(&self) -> i32 {
        self.half
    }

    pub fn render(&self, outer: &AABB, node: Point, data: &mut dyn Data) {
        let start = node - self.half;
        let aabb = AABB::new(start, self.size);
        self.component.render(data, outer, &aabb)
    }
}

impl Default for NodeStyle {
    fn default() -> Self {
        NodeStyle::default_with_size(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texture_generation::generation::component::rendering::RenderingComponent;
    use texture_generation::generation::data::{Data, RuntimeData};
    use texture_generation::math::color::{BLACK, RED};

    #[test]
    fn test_render_node() {
        let component = RenderingComponent::new_fill_area("corner", RED, 9);
        let node_style = NodeStyle::new(2, component);
        let mut data = RuntimeData::new(Size::new(6, 5), BLACK);

        node_style.render(&data.get_aabb(), Point::new(3, 2), &mut data);

        #[rustfmt::skip]
        let result = vec![
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK,   RED,   RED, BLACK, BLACK,
            BLACK, BLACK,   RED,   RED, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK, BLACK, BLACK, BLACK,
        ];

        assert_eq!(data.get_color_data(), &result);

        #[rustfmt::skip]
        let depth = vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 9, 9, 0, 0,
            0, 0, 9, 9, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(data.get_depth_data(), &depth);
    }
}
