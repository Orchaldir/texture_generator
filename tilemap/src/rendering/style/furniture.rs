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
use texture_generation::math::size::Size;
use texture_generation::utils::resource::Resource;

#[derive(Clone, Debug, PartialEq)]
/// How does the furniture look like?
pub struct FurnitureStyle {
    name: String,
    component: Component,
    front: FrontStyle,
}

impl FurnitureStyle {
    pub fn new<S: Into<String>>(name: S, component: Component) -> Self {
        FurnitureStyle {
            name: name.into(),
            component,
            front: FrontStyle::One(1),
        }
    }

    /// Renders the furniture into the [`Texture`] in the area defined by [`Data`].
    pub fn render(&self, resources: &Resources, texture: &mut Texture, data: &Data) {
        let thickness = self.front.get_thickness(resources);

        if thickness > 0 {
            let size = data.get_inner().size();
            let start = data.get_inner().start();

            let size_component = Size::new(size.width(), size.height() - thickness);
            let size_front = Size::new(size.width(), thickness);
            let mut start_front = start;
            start_front.y += size_component.height() as i32;

            let aabb_component = AABB::new(start, size_component);
            let aabb_front = AABB::new(start_front, size_front);

            self.component
                .generate(texture, &data.transform(aabb_component));
            self.front
                .render_horizontal(resources, &data.transform(aabb_front), true, texture);
        } else {
            self.component.generate(texture, &data);
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

        Self::new("default", Component::Rendering(Box::new(component)))
    }
}

impl Resource for FurnitureStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
