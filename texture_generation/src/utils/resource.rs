use anyhow::Result;

pub trait ResourceDefinition {
    type R: Default;

    fn convert(&self, size: u32) -> Result<Self::R>;
}

pub struct ResourceManager<T> {
    default: T,
    resources: Vec<T>,
}

impl<T> ResourceManager<T> {
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
}

impl<T: Default> Default for ResourceManager<T> {
    fn default() -> Self {
        ResourceManager::new(Vec::default(), T::default())
    }
}

pub fn into_manager<T: ResourceDefinition>(definitions: &[T], size: u32) -> ResourceManager<T::R> {
    let textures: Vec<T::R> = definitions
        .iter()
        .filter_map(|d| d.convert(size).ok())
        .collect();

    ResourceManager::new(textures, T::R::default())
}
