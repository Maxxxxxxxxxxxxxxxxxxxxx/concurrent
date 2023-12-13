use client_server::*;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    thread::sleep,
};

fn interrogate_results(result_timestamp: &mut u64, buf: &mut String) -> Result<(), Box<dyn Error>> {
    let new_timestamp = get_file_timestamp(RESULTS_PATH)?;
    if new_timestamp != *result_timestamp {
        File::open(RESULTS_PATH)?.read_to_string(buf)?;
        // println!("Result: {}", buf);
        *result_timestamp = new_timestamp;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Buffers for reading and writing data
    let mut input_buffer = String::new();
    let mut result_buffer = String::new();

    File::open(RESULTS_PATH)?.read_to_string(&mut result_buffer)?;

    // get result timestamp
    let mut result_timestamp = get_file_timestamp(RESULTS_PATH)?;

    if !result_buffer.is_empty() {
        interrogate_results(&mut result_timestamp, &mut result_buffer)?;
    }

    input_buffer = String::new();

    interrogate_results(&mut result_timestamp, &mut result_buffer)?;

    let mut file = File::options().write(true).open("data.txt")?;

    // Read buffer from stdin
    std::io::stdin().read_line(&mut input_buffer)?;

    if !input_buffer.is_empty() {
        // Write buffer to file
        file.write_fmt(format_args!("{}", input_buffer.replace('\n', "")))?;
        println!("written buffer: \'{}\'", &input_buffer.replace('\n', ""));

        result_buffer = String::new();
    }

    sleep(std::time::Duration::from_millis(200));

    interrogate_results(&mut result_timestamp, &mut result_buffer)?;
    println!("Result: {}", &result_buffer);

    Ok(())
}
