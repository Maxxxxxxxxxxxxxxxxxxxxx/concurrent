use fifo::{cleanup, Query};
use nix::{sys::stat::Mode, unistd::mkfifo};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, OpenOptions},
    io::{Read, Write},
    thread::sleep,
    time::Duration,
};

fn write_response(path: &str, res: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).open(path)?;
    let _ = file.write(res.as_bytes());

    log::info!("Written response to client {}", &path);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    ctrlc::set_handler(move || {
        cleanup("server/fifo");
        std::process::exit(130);
    })?;

    if fs::metadata("client").is_err() {
        fs::create_dir("client")?;
    };

    if fs::metadata("server").is_err() {
        fs::create_dir("server")?;
    };

    if fs::metadata("server/fifo").is_ok() {
        fs::remove_file("server/fifo")?;
    };

    let mut person_list = HashMap::<usize, String>::new();

    person_list.insert(0, "Smith".to_string());
    person_list.insert(1, "Schlamberger".to_string());
    person_list.insert(2, "Nazwisko".to_string());

    mkfifo("server/fifo", Mode::all())?;

    log::info!("Created server files and dir structure");

    loop {
        log::info!("Loop started");

        let mut server_fifo = OpenOptions::new().read(true).open("server/fifo")?;
        let mut buf = "".to_owned();

        server_fifo.read_to_string(&mut buf)?;

        log::info!("Read buffer: {}", buf.trim_end());

        if let Ok(query) = Query::try_parse(buf.clone()) {
            let client_queue_path = query.client_path;
            let person = person_list.get(&query.content);

            sleep(Duration::from_millis(500));

            if let Some(res) = person {
                write_response(&client_queue_path, res)?;
            } else {
                write_response(&client_queue_path, "Nie ma")?;
            }
        } else {
            log::error!("Failed to parse query!");
        }
    }
}
