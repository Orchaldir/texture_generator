use crate::generation::data::Data;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub trait Random {
    /// Returns a reproducible random number of type usize based on `instance_id` & `index`.
    fn get_random_instance_usize(&self, data: &Data, max_value: usize, index: u32) -> usize {
        self.get_random_instance_u64(data, max_value as u64, index) as usize
    }

    /// Returns a reproducible random number of type u32 based on `instance_id` & `index`.
    fn get_random_instance_u32(&self, data: &Data, max_value: u32, index: u32) -> u32 {
        self.get_random_instance_u64(data, max_value as u64, index) as u32
    }

    /// Returns a reproducible random number of type u64 based on `instance_id` & `index`.
    fn get_random_instance_u64(&self, data: &Data, max_value: u64, index: u32) -> u64;
}

pub struct RandomWithHasher;

impl Random for RandomWithHasher {
    /// Returns a reproducible random number of type u64 based on `instance_id` & `index`.
    fn get_random_instance_u64(&self, data: &Data, max_value: u64, index: u32) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write_usize(data.get_instance_id());
        hasher.write_u32(index);
        hasher.finish() % max_value
    }
}

pub struct MockRandom(Vec<u64>);

impl Random for MockRandom {
    /// Returns a reproducible random number of type u64 based on `instance_id` & `index`.
    fn get_random_instance_u64(&self, _data: &Data, _max_value: u64, index: u32) -> u64 {
        self.0[index as usize % self.0.len()]
    }
}

/// Returns a reproducible random number of type usize based on `instance_id` & `index`.
pub fn get_random_instance_usize(data: &Data, max_value: usize, index: u32) -> usize {
    let mut hasher = DefaultHasher::new();
    hasher.write_usize(data.get_instance_id());
    hasher.write_u32(index);
    (hasher.finish() % (max_value as u64)) as usize
}

/// Returns a reproducible random number of type u32 based on `instance_id` & `index`.
pub fn get_random_instance_u32(data: &Data, max_value: u32, index: u32) -> u32 {
    let mut hasher = DefaultHasher::new();
    hasher.write_usize(data.get_instance_id());
    hasher.write_u32(index);
    (hasher.finish() % (max_value as u64)) as u32
}
