use crate::rendering::style::edge::EdgeStyle;
use crate::rendering::style::node::NodeStyle;

#[derive(Clone, Debug, PartialEq)]
/// Determines how a wall is rendered.
pub struct WallStyle<T> {
    name: String,
    /// The style of a wall between or without nodes.
    edge_style: EdgeStyle,
    /// The optional style of a node between 2 wall segments in the same direction.
    node_style: Option<T>,
    /// The style of corners.
    corner_style: T,
}

impl<T> WallStyle<T> {
    pub fn default(thickness: u32, default_node_style: T) -> WallStyle<T> {
        Self::new(
            "default",
            EdgeStyle::default(thickness),
            None,
            default_node_style,
        )
    }

    pub fn new<S: Into<String>>(
        name: S,
        edge_style: EdgeStyle,
        node_style: Option<T>,
        corner_style: T,
    ) -> WallStyle<T> {
        WallStyle {
            name: name.into(),
            edge_style,
            node_style,
            corner_style,
        }
    }

    pub fn get_edge_style(&self) -> &EdgeStyle {
        &self.edge_style
    }

    pub fn get_node_style(&self) -> &Option<T> {
        &self.node_style
    }

    pub fn get_corner_style(&self) -> &T {
        &self.corner_style
    }

    pub fn is_greater(&self, other: &WallStyle<T>) -> bool {
        self.edge_style.get_thickness() >= other.edge_style.get_thickness()
    }
}

impl Default for WallStyle<NodeStyle> {
    fn default() -> Self {
        WallStyle::default(1, NodeStyle::default())
    }
}
