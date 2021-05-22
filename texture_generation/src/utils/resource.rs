use anyhow::Result;
use std::collections::HashMap;

pub trait Resource: Default {
    fn get_name(&self) -> &str;
}

pub trait ResourceDefinition {
    type R: Resource;

    fn convert(&self, name: &str, size: u32) -> Result<Self::R>;
}

pub trait ResourceLookup<T> {
    fn lookup(&self, resource_type: T, name: &str) -> Option<usize>;
}

pub struct ResourceManager<T: Resource> {
    default: T,
    resources: Vec<T>,
}

impl<T: Resource> ResourceManager<T> {
    pub fn new(resources: Vec<T>, default: T) -> ResourceManager<T> {
        ResourceManager { resources, default }
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn get(&self, id: usize) -> &T {
        self.resources.get(id).unwrap_or(&self.default)
    }

    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.resources
            .iter()
            .enumerate()
            .find(|(_i, r)| r.get_name().eq(name))
            .map(|(i, _r)| i)
    }
}

impl<T: Resource> Default for ResourceManager<T> {
    fn default() -> Self {
        ResourceManager::new(Vec::default(), T::default())
    }
}

pub fn into_manager<T: ResourceDefinition>(
    definitions: &HashMap<String, T>,
    size: u32,
) -> ResourceManager<T::R> {
    let resources: Vec<T::R> = definitions
        .iter()
        .filter_map(|(name, d)| match d.convert(name, size) {
            Ok(resource) => Some(resource),
            Err(error) => {
                eprintln!("Error: {:?}", error);
                None
            }
        })
        .collect();

    ResourceManager::new(resources, T::R::default())
}
