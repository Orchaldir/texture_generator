use crate::rendering::resource::Resources;
use crate::rendering::style::front::FrontStyle;
use crate::tilemap::Side;
use texture_generation::generation::component::rendering::color::ColorSelector;
use texture_generation::generation::component::rendering::depth_factory::DepthFactory;
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::component::Component;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::color::PINK;
use texture_generation::math::shape_factory::ShapeFactory;
use texture_generation::math::size::Size;
use texture_generation::utils::resource::Resource;

#[derive(Clone, Debug, PartialEq)]
/// How does the furniture look like?
pub struct FurnitureStyle {
    name: String,
    depth: u8,
    horizontal_component: Component,
    vertical_component: Component,
    front: FrontStyle,
}

impl FurnitureStyle {
    pub fn new<S: Into<String>>(
        name: S,
        depth: u8,
        component: Component,
        front: FrontStyle,
    ) -> Self {
        FurnitureStyle {
            name: name.into(),
            depth,
            horizontal_component: component.clone(),
            vertical_component: component.flip(),
            front,
        }
    }

    /// Renders the furniture into the [`Texture`] in the area defined by [`Data`].
    pub fn render(
        &self,
        resources: &Resources,
        texture: &mut Texture,
        data: &Data,
        front_side: Side,
    ) {
        texture.set_base_depth(self.depth);

        let thickness = self.front.get_thickness(resources);

        if thickness > 0 {
            let aabbs = calculate_aabbs(data, front_side, thickness);

            match front_side {
                Side::Top => self.render_horizontal(resources, texture, data, aabbs, false),
                Side::Left => self.render_vertical(resources, texture, data, aabbs, false),
                Side::Bottom => self.render_horizontal(resources, texture, data, aabbs, true),
                Side::Right => self.render_vertical(resources, texture, data, aabbs, true),
            }
        } else {
            match front_side {
                Side::Top | Side::Bottom => self.horizontal_component.generate(texture, &data),
                Side::Left | Side::Right => self.vertical_component.generate(texture, &data),
            }
        }
    }

    fn render_horizontal(
        &self,
        resources: &Resources,
        texture: &mut Texture,
        data: &Data,
        aabbs: (AABB, AABB),
        is_front: bool,
    ) {
        let (aabb_component, aabb_front) = aabbs;

        self.horizontal_component
            .generate(texture, &data.transform(aabb_component));
        self.front
            .render_horizontal(resources, &data.transform(aabb_front), is_front, texture);
    }

    fn render_vertical(
        &self,
        resources: &Resources,
        texture: &mut Texture,
        data: &Data,
        aabbs: (AABB, AABB),
        is_front: bool,
    ) {
        let (aabb_component, aabb_front) = aabbs;

        self.vertical_component
            .generate(texture, &data.transform(aabb_component));
        self.front
            .render_vertical(resources, &data.transform(aabb_front), is_front, texture);
    }
}

fn calculate_aabbs(data: &Data, front_side: Side, thickness: u32) -> (AABB, AABB) {
    let size = data.get_inner().size();
    let start = data.get_inner().start();

    match front_side {
        Side::Top => {
            let size_component = Size::new(size.width(), size.height() - thickness);
            let size_front = Size::new(size.width(), thickness);
            let mut start_component = start;
            start_component.y += thickness as i32;

            let aabb_component = AABB::new(start_component, size_component);
            let aabb_front = AABB::new(start, size_front);

            (aabb_component, aabb_front)
        }
        Side::Left => {
            let size_component = Size::new(size.width() - thickness, size.height());
            let size_front = Size::new(thickness, size.height());
            let mut start_component = start;
            start_component.x += thickness as i32;

            (
                AABB::new(start_component, size_component),
                AABB::new(start, size_front),
            )
        }
        Side::Bottom => {
            let size_component = Size::new(size.width(), size.height() - thickness);
            let size_front = Size::new(size.width(), thickness);
            let mut start_front = start;
            start_front.y += size_component.height() as i32;

            let aabb_component = AABB::new(start, size_component);
            let aabb_front = AABB::new(start_front, size_front);

            (aabb_component, aabb_front)
        }
        Side::Right => {
            let size_component = Size::new(size.width() - thickness, size.height());
            let size_front = Size::new(thickness, size.height());
            let mut start_front = start;
            start_front.x += size_component.width() as i32;

            (
                AABB::new(start, size_component),
                AABB::new(start_front, size_front),
            )
        }
    }
}

impl Default for FurnitureStyle {
    fn default() -> Self {
        let color_selector = ColorSelector::ConstantColor(PINK);
        let depth_factory = DepthFactory::Uniform(100);
        let component = RenderingComponent::new_shape_with_depth(
            ShapeFactory::RoundedRectangle(0.3),
            color_selector,
            depth_factory,
        );

        Self::new(
            "default",
            100,
            Component::Rendering(Box::new(component)),
            FrontStyle::None,
        )
    }
}

impl Resource for FurnitureStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
