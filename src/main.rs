use core::str;
use std::{env, fs::File, io::Read, io::Result};

use day1::solve_day_1;
use day1p2::solve_day_1_part_2;
use day2::solve_day_2;
use day2p2::solve_day_2_part_2;

mod day1;
mod day1p2;
mod day2;
mod day2p2;

pub fn get_file_text(file_name: &str) -> Result<String> {
    let current_dir = env::current_dir()?;

    let file_path = current_dir.join(file_name);

    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() -> Result<()> {
    solve_day_1()?;
    solve_day_1_part_2()?;
    solve_day_2()?;
    solve_day_2_part_2()?;

    Ok(())
}
