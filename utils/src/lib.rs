use std::fs::{self, DirEntry};
use std::path::Path;

use anyhow::{Context, Result};
use miniz_oxide::inflate::DecompressError;
use serde::Deserializer;
use serde_pickle::Value as PickleValue;
mod pickle;
pub use pickle::IntoSubValue;

// Get files in directory, given directory path (only direct childs of the directory)
#[cfg(not(target_arch = "wasm32"))]
pub fn parse_dir<P: AsRef<Path>>(path: P) -> Result<Vec<DirEntry>> {
    let file_paths = fs::read_dir(path).with_context(|| "failed to read dir")?;

    Ok(file_paths
        .filter_map(|entry| entry.ok().filter(|entry| entry.path().is_file()))
        .collect())
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum BoolableValue {
    Bool(bool),
    Int(i32),
}

/// WoT sometimes uses int and boolean interchangeably for the same field and we can't have that
pub fn bool_to_int<'de, D>(de: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let val: BoolableValue = serde::de::Deserialize::deserialize(de)?;

    match val {
        BoolableValue::Bool(val) => Ok(val as i32),
        BoolableValue::Int(val) => Ok(val),
    }
}

pub fn load_pickle(input: &[u8]) -> Result<PickleValue, DataError> {
    let result = serde_pickle::value_from_slice(input, Default::default())?;

    Ok(result)
}

pub fn decompress_vec<E>(compressed: &[u8], f: fn(DecompressError) -> E) -> Result<Vec<u8>, E> {
    miniz_oxide::inflate::decompress_to_vec_zlib(compressed).map_err(|err| f(err))
}

#[derive(thiserror::Error, Debug)]
pub enum DataError {
    #[error("Decompressed failed: {0}")]
    DecompressionFaliure(String),

    #[error("{0}")]
    PickleParseError(#[from] serde_pickle::Error),

    #[error("{0}")]
    Other(String),
}

/// `[0, 9, 15, 0]` => `"0_9_15_0"`
pub fn version_as_string(version: [u16; 4]) -> String {
    version.map(|x| x.to_string()).join("_")
}

/// `"0_9_15_0"` => `[0, 9, 15, 0]`  
pub fn version_string_as_arr(version: String) -> Option<[u16; 4]> {
    let vec: Option<Vec<u16>> = version.split('_').map(|v| v.parse().ok()).collect();

    if let Some(vec) = vec {
        if vec.len() == 4 {
            Some([vec[0], vec[1], vec[2], vec[3]])
        } else {
            None
        }
    } else {
        None
    }
}
