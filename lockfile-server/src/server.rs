use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    process::exit,
    thread::sleep,
    time::Duration,
};

use lockfile_server::get_file_timestamp;
use log::info;

const SERVER_BUFFER_PATH: &str = "server/buffer";

fn main() -> Result<(), Box<dyn Error>> {
    // Create buffer in read-only mode
    let server_buffer = OpenOptions::new()
        .read(true)
        .create(true)
        .open(SERVER_BUFFER_PATH)?;

    // CtrlC handler. Removes lockfile
    ctrlc::set_handler(move || {
        let _ = fs::remove_file("server/lockfile");
        exit(130);
    })?;

    // Watch server buffer for client connection
    loop {
        let server_timestamp = get_file_timestamp(SERVER_BUFFER_PATH)?;

        // Constant comparison of last file write timestamp
        if get_file_timestamp(SERVER_BUFFER_PATH)? != server_timestamp {
            // Read lines from file with BufReader
            // First line is path to client file, used for response
            let reader = BufReader::new(&server_buffer);
            let lines = reader
                .lines()
                .map(|line| line.unwrap())
                .collect::<Vec<String>>();

            // Get client file path from first line
            let client_file_path = &lines[0];

            // Construct response content from leftover lines
            let response_content: String = lines[1..].to_vec().join("\n");
            let _ = File::create(client_file_path)?.write(response_content.as_bytes())?;

            info!("Written response to client");

            sleep(Duration::from_millis(300));
            let _ = fs::remove_file("server/lockfile");
        }
    }
}
