use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("file")?;

    let mut input_buffer = String::new();

    // clear file contents
    f.set_len(0)?;

    std::io::stdin().read_line(&mut input_buffer)?;

    f.write_all(input_buffer.as_bytes())?;

    Ok(())
}
