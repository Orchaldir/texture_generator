use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use texture_generation::definition::convert;
use tilemap::rendering::style::front::FrontStyle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FrontDefinition {
    None,
    One(usize),
    Repeat { step: u32, door_id: usize },
    Split(Vec<(u32, Option<usize>)>),
}

impl FrontDefinition {
    pub fn convert(&self, parent: &str, factor: f32) -> Result<FrontStyle> {
        Ok(match self {
            FrontDefinition::None => FrontStyle::None,
            FrontDefinition::One(door_id) => FrontStyle::One(*door_id),
            FrontDefinition::Repeat { step, door_id } => {
                FrontStyle::new_repeat(convert(*step, factor), *door_id)
                    .context(format!("Failed to create '{}.Repeat'", parent))?
            }
            FrontDefinition::Split(entries) => FrontStyle::new_split(entries.clone())
                .context(format!("Failed to create '{}.Split'", parent))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_none() {
        assert_eq!(
            FrontDefinition::None.convert("test", 2.0).unwrap(),
            FrontStyle::None
        );
    }

    #[test]
    fn test_convert_one() {
        assert_eq!(
            FrontDefinition::One(42).convert("test", 2.0).unwrap(),
            FrontStyle::One(42)
        );
    }

    #[test]
    fn test_convert_repeat() {
        let definition = FrontDefinition::Repeat {
            step: 10,
            door_id: 5,
        };
        let style = FrontStyle::Repeat {
            step: 25,
            door_id: 5,
        };
        assert_eq!(definition.convert("test", 2.5).unwrap(), style);
    }

    #[test]
    fn test_convert_split() {
        let definition = FrontDefinition::Split(vec![(3, Some(10)), (3, None)]);
        let style = FrontStyle::Split(vec![(0.5, Some(10)), (0.5, None)]);
        assert_eq!(definition.convert("test", 2.0).unwrap(), style);
    }
}
