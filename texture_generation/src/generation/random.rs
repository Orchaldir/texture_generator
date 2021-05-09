use crate::generation::data::Data;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

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
