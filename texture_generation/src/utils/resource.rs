use anyhow::Result;
use std::collections::HashMap;

pub trait ResourceDefinition {
    type R: Default;

    fn convert(&self, size: u32) -> Result<Self::R>;
}

pub struct ResourceManager<T> {
    default: T,
    resources: HashMap<String, T>,
}

impl<T> ResourceManager<T> {
    pub fn new(resources: HashMap<String, T>, default: T) -> ResourceManager<T> {
        ResourceManager { resources, default }
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn get(&self, id: usize) -> &T {
        self.resources.get("id").unwrap_or(&self.default)
    }
}

impl<T: Default> Default for ResourceManager<T> {
    fn default() -> Self {
        ResourceManager::new(HashMap::default(), T::default())
    }
}

pub fn into_manager<T: ResourceDefinition>(
    definitions: &HashMap<String, T>,
    size: u32,
) -> ResourceManager<T::R> {
    let textures: HashMap<String, T::R> = definitions
        .iter()
        .filter_map(|(name, d)| {
            info!("name={}", name);
            match d.convert(size) {
                Ok(resource) => Some((name.clone(), resource)),
                Err(error) => {
                    eprintln!("Error: {:?}", error);
                    None
                }
            }
        })
        .collect();

    ResourceManager::new(textures, T::R::default())
}
