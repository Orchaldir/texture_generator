use crate::generation::component::layout::brick::BrickPattern;
use crate::generation::component::layout::herringbone::HerringbonePattern;
use crate::generation::component::layout::random_ashlar::RandomAshlarPattern;
use crate::generation::component::layout::repeat::RepeatLayout;
use crate::generation::component::layout::split::SplitLayout;
use crate::generation::data::texture::Texture;
use crate::generation::data::Data;

pub mod brick;
pub mod herringbone;
pub mod random_ashlar;
pub mod repeat;
pub mod split;

#[svgbobdoc::transform]
#[derive(Clone, Debug, PartialEq)]
/// Generates a layout,
pub enum LayoutComponent {
    BrickWall(BrickPattern),
    Herringbone(HerringbonePattern),
    Mock(u32),
    RandomAshlar(RandomAshlarPattern),
    Repeat(RepeatLayout),
    Split(SplitLayout),
}

impl LayoutComponent {
    /// Flips between horizontal & vertical mode.
    pub fn flip(&self) -> LayoutComponent {
        match self {
            LayoutComponent::BrickWall(..) => self.clone(),
            LayoutComponent::Herringbone(..) => self.clone(),
            LayoutComponent::Mock(_id) => self.clone(),
            LayoutComponent::RandomAshlar(..) => self.clone(),
            LayoutComponent::Repeat(repeat) => LayoutComponent::Repeat(repeat.flip()),
            LayoutComponent::Split(split) => LayoutComponent::Split(split.flip()),
        }
    }

    /// Generates the component in the area defined by the [`AABB`].
    pub fn generate(&self, texture: &mut Texture, data: &Data) {
        let combined = data.combine();

        match self {
            LayoutComponent::BrickWall(pattern) => pattern.generate(texture, combined),
            LayoutComponent::Herringbone(pattern) => pattern.generate(texture, &combined),
            LayoutComponent::Mock(id) => info!("Generate layout mock {}", *id),
            LayoutComponent::RandomAshlar(pattern) => pattern.generate(texture, combined),
            LayoutComponent::Repeat(repeat) => repeat.generate(texture, combined),
            LayoutComponent::Split(split) => split.generate(texture, combined),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::component::border::BorderComponent;
    use crate::generation::component::rendering::RenderingComponent;
    use crate::generation::component::Component;
    use crate::math::color::RED;
    use crate::math::shape_factory::ShapeFactory::Rectangle;

    pub fn create_component() -> Component {
        let renderer = RenderingComponent::new_shape(Rectangle, RED, 200);
        let rendering_component = Component::Rendering(Box::new(renderer));
        let border = BorderComponent::new_uniform(1, rendering_component);
        Component::Border(Box::new(border))
    }
}
