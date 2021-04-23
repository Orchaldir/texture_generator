use crate::math::size::Size;
use crate::utils::error::DefinitionError;
use serde::de::DeserializeOwned;
use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::path::Path;

pub mod generation;
pub mod math;

pub fn convert(value: u32, factor: f32) -> u32 {
    (value as f32 * factor) as u32
}

pub fn convert_size(value: &Size, factor: f32) -> Size {
    Size::new(
        convert(value.width(), factor),
        convert(value.height(), factor),
    )
}

pub fn read_dir<T: DeserializeOwned>(dir: &Path) -> Vec<T> {
    if !dir.is_dir() {
        warn!(
            "Couldn't read definitions, because the path {:?} is not a directory!",
            dir
        );
        return Vec::default();
    }

    let mut results = Vec::new();

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                read_entry(&mut results, entry);
            }
        }
        Err(error) => warn!("Couldn't read directory {:?}, because of {:?}", dir, error),
    }

    results
}

fn read_entry<T: DeserializeOwned>(results: &mut Vec<T>, entry: Result<DirEntry, Error>) {
    match entry {
        Ok(entry) => {
            let path = entry.path();

            if !path.is_file() {
                info!("Skip entry {:?}, because it is not a file", path);
                return;
            }

            match read(&path) {
                Ok(definition) => results.push(definition),
                Err(error) => warn!(
                    "Couldn't read definition {:?}, because of {:?}",
                    path, error
                ),
            }
        }
        Err(error) => warn!("Couldn't read entry, because of {:?}", error),
    }
}

pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T, DefinitionError> {
    let string = fs::read_to_string(path)?;
    let data: T = serde_yaml::from_str(&string)?;
    Ok(data)
}
