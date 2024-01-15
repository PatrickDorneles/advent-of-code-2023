use std::io;

use crate::get_file_text;

pub fn solve_day_1_part_2() -> io::Result<()> {
    let text = get_file_text("src/day1.input")?;

    let original_text = String::from(text);

    let replaced_text = original_text
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    let rows = replaced_text.trim().split("\n");

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

    println!("Day_1_Part_2: {}", sum);

    return Ok(());
}
