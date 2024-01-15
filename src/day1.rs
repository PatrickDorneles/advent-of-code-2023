use std::io;

use crate::get_file_text;

pub fn solve_day_1() -> io::Result<()> {
    let text = get_file_text("src/day1.input")?;

    let rows = text.split("\n");

    let mut sum = 0;

    for row in rows {
        let mut first_num_char: char = '_';
        let mut last_num_char: char = '_';

        for char in row.chars() {
            let convert_result = char.to_string().parse::<i32>();

            match convert_result {
                Ok(_) => {
                    first_num_char = char;
                    break;
                }
                Err(_) => continue,
            }
        }

        for char in row.chars().rev() {
            let convert_result = char.to_string().parse::<i32>();

            match convert_result {
                Ok(_) => {
                    last_num_char = char;
                    break;
                }
                Err(_) => continue,
            }
        }

        match format!("{}{}", first_num_char, last_num_char).parse::<i32>() {
            Ok(val) => sum += val,
            Err(_) => println!("Could not parse number!"),
        };
    }

    println!("{}", sum);

    return Ok(());
}
