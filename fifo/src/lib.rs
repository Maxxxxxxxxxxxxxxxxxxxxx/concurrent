use std::process::Command;

pub struct Query {
    pub client_path: String,
    pub content: usize,
}

impl Query {
    pub fn as_string(&self) -> String {
        format!("{} {}", &self.client_path, &self.content)
    }

    pub fn try_parse(query_string: String) -> Result<Self, ()> {
        let parts = query_string.split(' ').collect::<Vec<&str>>();
        if parts.len() == 2 {
            Ok(Self {
                client_path: parts[0].into(),
                content: usize::from_str_radix(parts[1].trim_end(), 10).unwrap_or(2137),
            })
        } else {
            Err(())
        }
    }
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
