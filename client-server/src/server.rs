#![allow(unused)]

use client_server::*;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    time::SystemTime,
};

fn do_something(num: i32) -> i32 {
    num + 5
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut data_timestamp = get_file_timestamp(DATA_PATH)?;

    loop {
        let new_timestamp = get_file_timestamp(DATA_PATH)?;

        // Interrogate file based on timestamp and read data
        if new_timestamp != data_timestamp {
            data_timestamp = new_timestamp;

            let mut file = File::open(DATA_PATH)?;
            let mut out = File::create("result.txt")?;

            println!("File timestamp changed. Opened file");

            file = File::open(DATA_PATH)?;
            file.read_to_string(&mut buffer);
            let number: i32 = str::parse(&buffer)?;

            out.write_fmt(format_args!("{}", do_something(number)));

            // Clear buffer
            buffer = String::new();

            println!("Written result to file");
        }
    }
}
