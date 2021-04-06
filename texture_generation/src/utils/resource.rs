pub struct ResourceManager<T> {
    resources: Vec<T>,
}

impl<T> ResourceManager<T> {
    pub fn new(resources: Vec<T>) -> ResourceManager<T> {
        ResourceManager { resources }
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.resources.get(id)
    }
}
