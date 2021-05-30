use texture_generation::generation::component::rendering::color::ColorSelector;
use texture_generation::generation::component::rendering::depth_factory::DepthFactory;
use texture_generation::generation::component::rendering::RenderingComponent;
use texture_generation::generation::component::Component;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;
use texture_generation::math::color::PINK;
use texture_generation::math::shape_factory::ShapeFactory;
use texture_generation::utils::resource::Resource;

#[derive(Clone, Debug, PartialEq)]
/// How does the furniture look like?
pub struct FurnitureStyle {
    name: String,
    component: Component,
}

impl FurnitureStyle {
    pub fn new<S: Into<String>>(name: S, component: Component) -> Self {
        FurnitureStyle {
            name: name.into(),
            component,
        }
    }

    /// Renders the furniture into the [`Texture`] in the area defined by [`Data`].
    pub fn render(&self, texture: &mut Texture, data: &Data) {
        self.component.generate(texture, &data);
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
