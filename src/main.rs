use core::str;
use std::{env, fs::File, io::Read, io::Result};

mod day1;

pub fn get_file_text(file_name: &str) -> Result<String> {
    let current_dir = env::current_dir()?;

    let file_path = current_dir.join(file_name);

    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() -> Result<()> {
    println!("{}", get_file_text("src/test_file")?);
    return Ok(());
}
