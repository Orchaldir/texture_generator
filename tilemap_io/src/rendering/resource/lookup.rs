use serde::{Deserialize, Serialize};
use texture_generation::utils::resource::ResourceLookup;

pub enum ResourceType {
    Door,
    Node,
    Texture,
    Wall,
    Window,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TilemapResourceLookup {
    doors: Vec<String>,
    nodes: Vec<String>,
    textures: Vec<String>,
    walls: Vec<String>,
    windows: Vec<String>,
}

impl TilemapResourceLookup {
    pub fn new(
        doors: Vec<String>,
        nodes: Vec<String>,
        textures: Vec<String>,
        walls: Vec<String>,
        windows: Vec<String>,
    ) -> Self {
        Self {
            doors,
            nodes,
            textures,
            walls,
            windows,
        }
    }
}

impl ResourceLookup<ResourceType> for TilemapResourceLookup {
    fn lookup(&self, resource_type: ResourceType, name: &str) -> Option<usize> {
        let resources = match resource_type {
            ResourceType::Door => &self.doors,
            ResourceType::Node => &self.nodes,
            ResourceType::Texture => &self.textures,
            ResourceType::Wall => &self.walls,
            ResourceType::Window => &self.windows,
        };
        lookup(resources, name)
    }
}

fn lookup(resources: &[String], name: &str) -> Option<usize> {
    resources.iter().position(|r| r.eq(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::resource::lookup::ResourceType::*;

    #[test]
    fn test_lookup() {
        let doors = prepare(vec!["D0", "D2", "D4"]);
        let wall = prepare(vec!["w10", "w11", "w111"]);
        let lookup = TilemapResourceLookup::new(doors, Vec::new(), Vec::new(), wall, Vec::new());

        assert_eq!(lookup.lookup(Door, "D0"), Some(0));
        assert_eq!(lookup.lookup(Door, "D1"), None);
        assert_eq!(lookup.lookup(Door, "D2"), Some(1));
        assert_eq!(lookup.lookup(Door, "D3"), None);
        assert_eq!(lookup.lookup(Door, "D4"), Some(2));

        assert_eq!(lookup.lookup(Wall, "w10"), Some(0));
        assert_eq!(lookup.lookup(Wall, "w11"), Some(1));
        assert_eq!(lookup.lookup(Wall, "w111"), Some(2));

        assert_eq!(lookup.lookup(Node, "D0"), None);
        assert_eq!(lookup.lookup(Texture, "D0"), None);
        assert_eq!(lookup.lookup(Wall, "D0"), None);
        assert_eq!(lookup.lookup(Window, "D0"), None);

        assert_eq!(lookup.lookup(Door, "w10"), None);
        assert_eq!(lookup.lookup(Node, "w10"), None);
        assert_eq!(lookup.lookup(Texture, "w10"), None);
        assert_eq!(lookup.lookup(Window, "w10"), None);
    }

    fn prepare(names: Vec<&str>) -> Vec<String> {
        names.into_iter().map(|s| s.into()).collect()
    }
}
