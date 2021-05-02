use crate::rendering::style::edge::EdgeStyle;
use crate::rendering::style::node::NodeStyle;
use texture_generation::generation::data::texture::Data;
use texture_generation::math::aabb::AABB;
use texture_generation::math::point::Point;

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
            EdgeStyle::default(thickness),
            EdgeStyle::default(thickness),
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
        outer: &AABB,
        node: Point,
        tile_size: u32,
        start_node: Option<&NodeStyle>,
        end_node: Option<&NodeStyle>,
        data: &mut dyn Data,
    ) {
        self.stool_style
            .render_horizontal(outer, node, tile_size, 0, start_node, end_node, data);
        self.pane_style
            .render_horizontal(outer, node, tile_size, 0, start_node, end_node, data);
    }

    pub fn render_vertical(
        &self,
        outer: &AABB,
        node: Point,
        tile_size: u32,
        start_node: Option<&NodeStyle>,
        end_node: Option<&NodeStyle>,
        data: &mut dyn Data,
    ) {
        self.stool_style
            .render_vertical(outer, node, tile_size, 0, start_node, end_node, data);
        self.pane_style
            .render_vertical(outer, node, tile_size, 0, start_node, end_node, data);
    }
}

impl Default for WindowStyle {
    fn default() -> Self {
        WindowStyle::default(1)
    }
}
