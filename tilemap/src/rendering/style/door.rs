use crate::rendering::style::edge::EdgeStyle;

#[derive(Clone, Debug, PartialEq)]
/// Determines how a door is rendered.
pub struct DoorStyle {
    name: String,
    /// The style of a movable part of the door.
    edge_style: EdgeStyle,
    is_centered: bool,
}

impl DoorStyle {
    pub fn default(thickness: u32) -> DoorStyle {
        Self::new("default", EdgeStyle::default(thickness).unwrap(), true)
    }

    pub fn new<S: Into<String>>(name: S, edge_style: EdgeStyle, is_centered: bool) -> DoorStyle {
        DoorStyle {
            name: name.into(),
            edge_style,
            is_centered,
        }
    }

    pub fn get_edge_style(&self) -> &EdgeStyle {
        &self.edge_style
    }

    pub fn get_offset(&self, wall_thickness: u32, is_front: bool) -> i32 {
        if self.is_centered {
            return 0;
        }

        let offset = (wall_thickness + self.edge_style.get_thickness()) as i32 / 2;

        if is_front {
            offset
        } else {
            -offset
        }
    }
}

impl Default for DoorStyle {
    fn default() -> Self {
        DoorStyle::default(1)
    }
}
