use crate::rendering::resource::Resources;
use texture_generation::generation::component::layout::repeat::calculate_steps;
use texture_generation::generation::data::texture::Texture;
use texture_generation::generation::data::Data;

#[derive(Clone, Debug, PartialEq)]
/// Doors & drawers in front of the furniture.
pub enum FrontStyle {
    None,
    One(usize),
    Repeat { step: u32, door_id: usize },
    Split(Vec<(f32, Option<usize>)>),
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
        let start = data.get_inner().start();
        let size = data.get_inner().size();
        let mut point = start;
        point.y += size.height() as i32 / 2;

        match self {
            FrontStyle::One(door_id) => {
                resources.door_styles.get(*door_id).render_horizontal(
                    data,
                    point,
                    (0, size.width()),
                    is_front,
                    texture,
                );
            }
            FrontStyle::Repeat { step, door_id } => {
                let door_style = resources.door_styles.get(*door_id);

                for step in calculate_steps(size.width(), *step) {
                    door_style.render_horizontal(data, point, (0, step), is_front, texture);
                    point.x += step as i32;
                }
            }
            FrontStyle::Split(entries) => {
                let length = size.width() as f32;

                for (factor, o) in entries {
                    let step = (length * *factor) as u32;

                    if let Some(door_id) = o {
                        resources.door_styles.get(*door_id).render_horizontal(
                            data,
                            point,
                            (0, step),
                            is_front,
                            texture,
                        );
                    }

                    point.x += step as i32;
                }
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
