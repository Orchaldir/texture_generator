use crate::rendering::style::edge::EdgeStyle;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;
use texture_generation::math::point::Point;
use texture_generation::utils::resource::Resource;

#[derive(Clone, Debug, PartialEq)]
/// Determines how a door is rendered.
pub struct WindowStyle {
    name: String,
    /// The style of the plate of glass.
    pane_style: EdgeStyle,
    /// The style of the horizontal board below the window.
    stool_style: EdgeStyle,
}

impl WindowStyle {
    pub fn default(thickness: u32) -> WindowStyle {
        Self::new(
            "default",
            EdgeStyle::default(thickness).unwrap(),
            EdgeStyle::default(thickness).unwrap(),
        )
    }

    pub fn new<S: Into<String>>(
        name: S,
        pane_style: EdgeStyle,
        stool_style: EdgeStyle,
    ) -> WindowStyle {
        WindowStyle {
            name: name.into(),
            pane_style,
            stool_style,
        }
    }

    pub fn render_horizontal(
        &self,
        data: &Data,
        node: Point,
        edge: (i32, u32),
        texture: &mut Texture,
    ) {
        self.stool_style
            .render_horizontal(data, node, edge, 0, texture);
        self.pane_style
            .render_horizontal(data, node, edge, 0, texture);
    }

    pub fn render_vertical(
        &self,
        data: &Data,
        node: Point,
        edge: (i32, u32),
        texture: &mut Texture,
    ) {
        self.stool_style
            .render_vertical(data, node, edge, 0, texture);
        self.pane_style
            .render_vertical(data, node, edge, 0, texture);
    }
}

impl Default for WindowStyle {
    fn default() -> Self {
        WindowStyle::default(1)
    }
}

impl Resource for WindowStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
