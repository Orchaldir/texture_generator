use crate::generation::data::Data;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::Hasher;

pub const COLOR_INDEX: u32 = 0;

const MAX_FOR_F32: u64 = 1000000;

#[derive(Clone, Debug, PartialEq)]
pub enum Random {
    Hash,
    Mock(Vec<u64>),
}

impl Random {
    /// Returns a reproducible random number between 0 and `max_value` of type usize based on `instance_id` & `index`.
    pub fn get_random_instance_usize(&self, data: &Data, max_value: usize, index: u32) -> usize {
        (self.next(data, index) % (max_value as u64)) as usize
    }

    /// Returns a reproducible random number between 0 and `max_value` of type u32 based on `instance_id` & `index`.
    pub fn get_random_instance_u32(&self, data: &Data, max_value: u32, index: u32) -> u32 {
        (self.next(data, index) % (max_value as u64)) as u32
    }

    /// Returns a reproducible random number between 0 and `max_value` of type f32 based on `instance_id` & `index`.
    pub fn get_random_instance_f32(&self, data: &Data, max_value: f32, index: u32) -> f32 {
        max_value * ((self.next(data, index) % MAX_FOR_F32) as f32 / MAX_FOR_F32 as f32)
    }

    /// Returns a reproducible random bool based on `instance_id` & `index`.
    pub fn get_random_instance_bool(&self, data: &Data, index: u32) -> bool {
        (self.next(data, index) % 2) != 0
    }

    /// Returns a reproducible random number based on [`Data`]'s `instance_id` & `index`.
    fn next(&self, data: &Data, index: u32) -> u64 {
        match self {
            Random::Hash => {
                let mut hasher = DefaultHasher::new();
                hasher.write_usize(data.get_instance_id());
                hasher.write_u32(index);
                hasher.finish()
            }
            Random::Mock(numbers) => {
                let index = data.get_instance_id() + index as usize;
                numbers[index % numbers.len()]
            }
        }
    }
}
