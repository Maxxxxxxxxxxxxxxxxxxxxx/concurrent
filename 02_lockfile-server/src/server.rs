use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    process::exit,
    thread::sleep,
    time::Duration,
};

use lockfile_server::get_file_timestamp;

const SERVER_BUFFER_PATH: &str = "server/buffer";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create server buffer
    let mut server_buffer = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(SERVER_BUFFER_PATH)?;

    // Initialize server timestamp
    let mut server_timestamp = get_file_timestamp(SERVER_BUFFER_PATH)?;

    // CtrlC handler. Removes lockfile
    ctrlc::set_handler(move || {
        let _ = fs::remove_file("server/lockfile");
        exit(130);
    })?;

    // Main loop
    // Watches server buffer for client connection
    loop {
        // Constant comparison of last file write timestamp
        if get_file_timestamp(SERVER_BUFFER_PATH)? != server_timestamp {
            server_timestamp = get_file_timestamp(SERVER_BUFFER_PATH)?;

            // Reopen the file
            server_buffer = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(SERVER_BUFFER_PATH)?;

            // Read lines from file with BufReader
            // First line is path to client file, used for response
            let reader = BufReader::new(&server_buffer);

            let lines = reader
                .lines()
                .map(|line| line.unwrap())
                .collect::<Vec<String>>();

            // Commit operation only if server buffer is not empty
            if !lines.is_empty() {
                // Get client file path from first line
                let client_file_path = &lines[0];

                // Construct response content from leftover lines
                let response_content: String = lines[1..].to_vec().join("\n");
                let _ = File::create(client_file_path)?.write(response_content.as_bytes())?;

                println!("Written response to client");

                // Truncate file for next client
                server_buffer.set_len(0)?;

                sleep(Duration::from_millis(300));
                let _ = fs::remove_file("server/lockfile");
            }
        }
    }
}
