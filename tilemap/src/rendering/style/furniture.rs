use crate::rendering::resource::Resources;
use crate::rendering::style::front::FrontStyle;
use texture_generation::generation::component::rendering::color::ColorSelector;
use texture_generation::generation::component::rendering::depth_factory::DepthFactory;
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::component::Component;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::color::PINK;
use texture_generation::math::shape_factory::ShapeFactory;
use texture_generation::math::side::Side;
use texture_generation::math::size::Size;
use texture_generation::utils::resource::Resource;

#[derive(Clone, Debug, PartialEq)]
pub enum FurnitureSize {
    Fill,
    Fixed(Size),
}

#[derive(Clone, Debug, PartialEq)]
/// How does the furniture look like?
pub struct FurnitureStyle {
    name: String,
    size: FurnitureSize,
    depth: u8,
    component: Component,
    front: FrontStyle,
}

impl FurnitureStyle {
    pub fn new<S: Into<String>>(
        name: S,
        size: FurnitureSize,
        depth: u8,
        component: Component,
        front: FrontStyle,
    ) -> Self {
        FurnitureStyle {
            name: name.into(),
            size,
            depth,
            component,
            front,
        }
    }

    /// Renders the furniture into the [`Texture`] in the area defined by [`Data`].
    pub fn render(&self, resources: &Resources, texture: &mut Texture, data: &Data) {
        texture.set_base_depth(self.depth);

        let aabb = self.calculate_aabb(data);
        let new_data = data.transform(aabb);
        let thickness = self.front.get_thickness(resources);

        if thickness > 0 {
            let (aabb_component, aabb_front) = calculate_aabbs(&new_data, thickness);

            self.component
                .generate(texture, &data.transform(aabb_component));
            self.front.render_horizontal(
                resources,
                &data.transform(aabb_front).make_vertical(),
                texture,
                false,
            );
        } else {
            self.component.generate(texture, &new_data);
        }
    }

    fn calculate_aabb(&self, data: &Data) -> AABB {
        let aabbs = data.get_aabbs();

        match self.size {
            FurnitureSize::Fill => aabbs.get_inner().clone(),
            FurnitureSize::Fixed(size) => {
                let inner = aabbs.get_inner();
                let center = inner.center();
                let start = center - size * 0.5;
                AABB::new(start, size)
            }
        }
    }
}

fn calculate_aabbs(data: &Data, thickness: u32) -> (AABB, AABB) {
    let size = data.get_aabbs().get_inner().size();
    let start = data.get_aabbs().get_inner().start();

    let size_component = Size::new(size.width() - thickness, size.height());
    let size_front = Size::new(thickness, size.height());
    let mut start_front = start;
    start_front.x += size_component.width() as i32;

    (
        AABB::new(start, size_component),
        AABB::new(start_front, size_front),
    )
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
            FurnitureSize::Fill,
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
