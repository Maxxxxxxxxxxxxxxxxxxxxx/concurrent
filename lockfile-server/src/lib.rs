use std::{error::Error, fmt::Display};

#[derive(Debug, Default)]
pub struct LockfileError;

// impl LockfileError {
//     pub fn new() -> LockfileError {
//         LockfileError {}
//     }
// }

impl Display for LockfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lockfile not found!")
    }
}

impl Error for LockfileError {}
