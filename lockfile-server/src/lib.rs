use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{atomic::AtomicU64, Arc, Mutex},
    thread,
    time::SystemTime,
};

pub fn get_file_timestamp(file_path: &str) -> Result<u64, Box<dyn Error>>
// where
    // P: AsRef<Path>,
{
    let metadata = fs::metadata(file_path)?;
    Ok(metadata
        .modified()?
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}

// Spawns independent process that removes a file on given path
// Executes regardless of main thread
pub fn cleanup(path: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("sleep 3 && rm {}", path)) // Sleep for 0.5s and then remove the file
        .spawn()
        .expect("Failed to start child process");
}

// Checks if lockfile exists
pub fn lockfile_exists() -> Result<(), ()> {
    let dir_contents = fs::read_dir("server");

    match dir_contents {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name == "lockfile" {
                            return Ok(());
                        }
                    }
                }
            }
            Err(())
        }
        Err(_) => Err(()),
    }
}
