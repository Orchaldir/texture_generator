use crate::rendering::style::edge::EdgeStyle;

#[derive(Clone, Debug, PartialEq)]
/// Determines how a door is rendered.
pub struct DoorStyle {
    name: String,
    /// The style of a movable part of the door.
    edge_style: EdgeStyle,
}

impl DoorStyle {
    pub fn new<S: Into<String>>(name: S, edge_style: EdgeStyle) -> DoorStyle {
        DoorStyle {
            name: name.into(),
            edge_style,
        }
    }

    pub fn get_edge_style(&self) -> &EdgeStyle {
        &self.edge_style
    }
}
