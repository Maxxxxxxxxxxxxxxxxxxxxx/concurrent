use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    process::{exit, Command},
    sync::Arc,
    thread::sleep,
    time::Duration,
};

use uuid::Uuid;

// Checks if function exists
fn lockfile_exists() -> Result<(), ()> {
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

// Spawns independent process that cleans up client file
// Executes regardless of main thread
fn cleanup(id: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("sleep 3 && rm client/{}", id)) // Sleep for 0.5s and then remove the file
        .spawn()
        .expect("Failed to start child process");
}

fn main() -> Result<(), Box<dyn Error>> {
    // Check for lockfile
    // 'Connection id' is also file name
    let conn_id = Uuid::new_v4().to_string();
    let id_arc = Arc::new(conn_id.clone());

    // Create client file
    let mut file = File::create(format!("client/{}", &conn_id))?;

    // Set Ctrl+C handler for cleaning up client files on exit
    ctrlc::set_handler(move || {
        let id = Arc::clone(&id_arc);
        cleanup(id.as_str());
        exit(130);
    })?;

    loop {
        // Constant check for lockfile in loop
        match lockfile_exists() {
            Ok(_) => {
                sleep(Duration::from_secs(2));
                println!("Server is busy...");
            }
            Err(_) => {
                // Create new buffer for user input
                let mut buffer = String::new();

                // Display prompt and read user input
                print!("> ");
                std::io::stdout().flush()?;
                std::io::stdin().read_line(&mut buffer)?;
                let byte_count = file.write(buffer.as_bytes())?;

                println!("Written {} bytes to #{}", byte_count, conn_id);

                // Spawn process to clean up client file after operation
                cleanup(&conn_id);

                return Ok(());
            }
        }
    }
}
