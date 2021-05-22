use crate::rendering::style::edge::EdgeStyle;
use texture_generation::utils::resource::Resource;

#[derive(Clone, Debug, PartialEq)]
/// Determines how a wall is rendered.
pub struct WallStyle {
    name: String,
    /// The style of a wall between or without nodes.
    edge_style: EdgeStyle,
    /// The optional style of a node between 2 wall segments in the same direction.
    node_style: Option<usize>,
    /// The style of corners.
    corner_style: usize,
}

impl WallStyle {
    pub fn default(thickness: u32) -> WallStyle {
        Self::new("default", EdgeStyle::default(thickness).unwrap(), None, 0)
    }

    pub fn new<S: Into<String>>(
        name: S,
        edge_style: EdgeStyle,
        node_style: Option<usize>,
        corner_style: usize,
    ) -> WallStyle {
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

    pub fn get_node_style(&self) -> Option<usize> {
        self.node_style
    }

    pub fn get_corner_style(&self) -> usize {
        self.corner_style
    }

    pub fn is_greater(&self, other: &WallStyle) -> bool {
        self.edge_style.get_thickness() >= other.edge_style.get_thickness()
    }
}

impl Default for WallStyle {
    fn default() -> Self {
        Self::default(1)
    }
}

impl Resource for WallStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
