use crate::rendering::resource::Resources;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;

#[derive(Clone, Debug, PartialEq)]
/// Doors & drawers in front of the furniture.
pub enum FrontStyle {
    None,
    One(usize),
    Repeat { size: u32, door_id: usize },
    Split(Vec<(u32, Option<usize>)>),
}

impl FrontStyle {
    pub fn get_thickness(&self, resources: &Resources) -> u32 {
        match self {
            FrontStyle::None => 0,
            FrontStyle::One(door_id) | FrontStyle::Repeat { door_id, .. } => {
                resources.door_styles.get(*door_id).get_thickness()
            }
            FrontStyle::Split(entries) => entries
                .iter()
                .map(|(_, o)| match o {
                    None => 0,
                    Some(id) => resources.door_styles.get(*id).get_thickness(),
                })
                .max()
                .unwrap_or_default(),
        }
    }

    pub fn render_horizontal(
        &self,
        resources: &Resources,
        data: &Data,
        is_front: bool,
        texture: &mut Texture,
    ) {
        match self {
            FrontStyle::One(door_id) => {
                let mut point = data.get_inner().start();
                let size = data.get_inner().size();
                point.y += size.height() as i32 / 2;

                resources.door_styles.get(*door_id).render_horizontal(
                    data,
                    point,
                    (0, size.width()),
                    is_front,
                    texture,
                );
            }
            _ => {}
        }
    }
}

impl Default for FrontStyle {
    fn default() -> Self {
        Self::None
    }
}
