use crate::rendering::resource::Resources;
use anyhow::{bail, Result};
use texture_generation::generation::component::layout::repeat::calculate_steps;
use texture_generation::generation::component::layout::split::convert_proportional;
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
    pub fn new_repeat(step: u32, door_id: usize) -> Result<Self> {
        if step < 1 {
            bail!("Step is is too small!");
        }

        Ok(FrontStyle::Repeat { step, door_id })
    }

    pub fn new_split(entries: Vec<(u32, Option<usize>)>) -> Result<Self> {
        let converted = convert_proportional(entries, "doors")?;
        let mut has_some = false;
        let mut was_last_none = false;

        for (_, option) in converted.iter() {
            if option.is_some() {
                if !has_some {
                    has_some = true;
                }
            } else if was_last_none {
                bail!("2 entries next to each other have no door!");
            } else {
                was_last_none = true;
            }
        }

        if !has_some {
            bail!("{} entries without a single door!", converted.len());
        }

        Ok(FrontStyle::Split(converted))
    }

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
        let start = data.get_aabbs().get_inner().start();
        let size = data.get_aabbs().get_inner().size();
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

    pub fn render_vertical(
        &self,
        resources: &Resources,
        data: &Data,
        is_front: bool,
        texture: &mut Texture,
    ) {
        let start = data.get_aabbs().get_inner().start();
        let size = data.get_aabbs().get_inner().size();
        let mut point = start;
        point.x += size.width() as i32 / 2;

        match self {
            FrontStyle::One(door_id) => {
                resources.door_styles.get(*door_id).render_vertical(
                    data,
                    point,
                    (0, size.height()),
                    is_front,
                    texture,
                );
            }
            FrontStyle::Repeat { step, door_id } => {
                let door_style = resources.door_styles.get(*door_id);

                for step in calculate_steps(size.height(), *step) {
                    door_style.render_vertical(data, point, (0, step), is_front, texture);
                    point.y += step as i32;
                }
            }
            FrontStyle::Split(entries) => {
                let length = size.height() as f32;

                for (factor, o) in entries {
                    let step = (length * *factor) as u32;

                    if let Some(door_id) = o {
                        resources.door_styles.get(*door_id).render_vertical(
                            data,
                            point,
                            (0, step),
                            is_front,
                            texture,
                        );
                    }

                    point.y += step as i32;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_new_repeat_with_too_small_step() {
        FrontStyle::new_repeat(0, 10).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_split_with_too_few_doors() {
        FrontStyle::new_split(vec![(2, Some(10))]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_split_with_proportion_is_zero() {
        FrontStyle::new_split(vec![(0, Some(10)), (3, Some(11))]).unwrap();
    }

    #[test]
    fn test_new_split_with_no_door() {
        let result = FrontStyle::new_split(vec![(3, Some(10)), (3, None)]);
        let style = FrontStyle::Split(vec![(0.5, Some(10)), (0.5, None)]);
        assert_eq!(result.unwrap(), style);
    }

    #[test]
    #[should_panic]
    fn test_new_split_with_no_doors() {
        FrontStyle::new_split(vec![(3, None), (3, None)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_split_with_2_nones_next_to_each_other() {
        FrontStyle::new_split(vec![(3, Some(10)), (3, None), (3, None)]).unwrap();
    }
}
