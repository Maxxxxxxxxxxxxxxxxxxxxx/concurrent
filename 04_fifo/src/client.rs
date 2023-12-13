use std::{
    error::Error,
    fs::OpenOptions,
    io::{Read, Write},
    sync::Arc,
};

use fifo::cleanup;
use nix::{sys::stat::Mode, unistd::mkfifo};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let client_id = Uuid::new_v4();
    let path = format!("client/{}", client_id);
    let path_arc = Arc::new(path.clone());

    ctrlc::set_handler(move || {
        cleanup(&path_arc);
        std::process::exit(130);
    })?;

    mkfifo(path.as_str(), Mode::all())?;

    let mut buffer = "".to_string();
    buffer.push_str(&path);
    buffer.push(' ');

    // Display prompt and read user input
    println!("Input query id");
    print!("> ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut buffer)?;

    let mut server_file = OpenOptions::new().write(true).open("server/fifo")?;
    let _ = server_file.write(buffer.as_bytes())?;

    log::info!("Written buffer to server fifo");

    drop(server_file);

    let mut client_fifo = OpenOptions::new().read(true).open(&path)?;

    log::info!("Awaiting response...");

    let mut b = "".to_owned();
    client_fifo.read_to_string(&mut b)?;

    log::info!("Read buffer from client fifo");

    if !b.is_empty() {
        println!("{}", b);
        return Ok(());
    }

    cleanup(&path);

    Ok(())
}
