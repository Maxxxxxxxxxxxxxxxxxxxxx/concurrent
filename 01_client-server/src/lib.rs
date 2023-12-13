#![allow(clippy::needless_question_mark)]

use std::{error::Error, fs, path::Path, time::SystemTime};

pub fn get_file_timestamp<P>(file_path: P) -> Result<u64, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let metadata = fs::metadata(file_path)?;
    Ok(metadata
        .modified()?
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}

pub const DATA_PATH: &str = "data.txt";
pub const RESULTS_PATH: &str = "result.txt";
