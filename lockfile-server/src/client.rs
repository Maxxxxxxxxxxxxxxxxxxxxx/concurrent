use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    process::exit,
    sync::Arc,
    thread::sleep,
    time::Duration,
};

use lockfile_server::*;
use uuid::Uuid;

fn handle_connection(file: &mut File, file_path: &str) -> Result<(), Box<dyn Error>> {
    // Create lockfile to lock connection
    File::create("server/lockfile")?;
    // Create new buffer for user input
    let mut buffer = String::new();
    // Open server buffer in write only mode
    let mut server_buffer_file = File::create("server/buffer")?;
    // Write client file path to server buffer at first line
    let _ = server_buffer_file.write(format!("{}\n", file_path).as_bytes())?;

    // Display prompt and read user input
    print!("> ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut buffer)?;
    let byte_count = server_buffer_file.write(buffer.as_bytes())?;

    println!("Written {} bytes to server buffer", byte_count);

    // Await server response to client file
    println!("Awaiting server response...");
    loop {
        let timestamp = get_file_timestamp(file_path)?;

        // Checks if timestamp of the file changes: if file gets written
        if get_file_timestamp(file_path)? != timestamp {
            // Create response buffer and read
            // client file content written by server into it
            let mut response_buff = String::new();
            File::open(file_path)?.read_to_string(&mut response_buff)?;

            println!("Server response: {}", response_buff);

            break;
        }
    }

    // Spawn process to clean up client file after operation
    cleanup(file_path);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Check for lockfile
    // 'Connection id' is also file name
    let conn_id = Uuid::new_v4().to_string();
    // Client file path
    let file_path = format!("client/{}", conn_id);
    let file_path_arc = Arc::new(file_path.clone());

    // Create client file
    let mut file = File::create(&file_path)?;

    // Set Ctrl+C handler for cleaning up client files on exit
    ctrlc::set_handler(move || {
        let path = Arc::clone(&file_path_arc);
        cleanup(&path);
        exit(130);
    })?;

    loop {
        // Constant check for lockfile in loop
        match lockfile_exists() {
            Ok(_) => {
                sleep(Duration::from_secs(2));
                println!("Server is busy...");
            }
            Err(_) => return handle_connection(&mut file, &file_path),
        }
    }
}
