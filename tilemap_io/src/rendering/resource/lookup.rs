use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLookup {
    doors: Vec<String>,
    nodes: Vec<String>,
    textures: Vec<String>,
    walls: Vec<String>,
    windows: Vec<String>,
}
