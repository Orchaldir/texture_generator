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
