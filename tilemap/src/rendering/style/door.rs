use crate::rendering::style::edge::EdgeStyle;
use crate::rendering::style::handle::HandleStyle;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;
use texture_generation::math::point::Point;
use texture_generation::utils::resource::Resource;

#[derive(Clone, Debug, PartialEq)]
/// Determines how a door is rendered.
pub struct DoorStyle {
    name: String,
    /// The style of a movable part of the door.
    edge_style: EdgeStyle,
    handle_style: Option<HandleStyle>,
    is_centered: bool,
}

impl DoorStyle {
    pub fn default(thickness: u32) -> DoorStyle {
        Self::new(
            "default",
            EdgeStyle::default(thickness).unwrap(),
            None,
            true,
        )
    }

    pub fn new<S: Into<String>>(
        name: S,
        edge_style: EdgeStyle,
        handle_style: Option<HandleStyle>,
        is_centered: bool,
    ) -> DoorStyle {
        DoorStyle {
            name: name.into(),
            edge_style,
            handle_style,
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

    pub fn render_horizontal(
        &self,
        data: &Data,
        node: Point,
        edge: (i32, u32),
        offset: i32,
        texture: &mut Texture,
    ) {
        if let Some(handle) = &self.handle_style {
            handle.render_horizontal(data, node, edge, offset, texture);
        }

        self.edge_style
            .render_horizontal(data, node, edge, offset, texture);
    }
}

impl Default for DoorStyle {
    fn default() -> Self {
        DoorStyle::default(1)
    }
}

impl Resource for DoorStyle {
    fn get_name(&self) -> &str {
        &self.name
    }
}
