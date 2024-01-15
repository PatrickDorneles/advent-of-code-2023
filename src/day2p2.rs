use core::panic;
use std::io;

use crate::get_file_text;

pub fn solve_day_2_part_2() -> io::Result<()> {
    let text = get_file_text("src/day2.input")?;

    let rows = text.trim().split("\n");

    let mut sum = 0;

    for row in rows {
        let split_result: [String; 2] = row
            .split(":")
            .map(String::from)
            .collect::<Vec<String>>()
            .try_into()
            .unwrap_or_else(|v: Vec<String>| {
                panic!("Expected 2 values but instead got {}", v.len())
            });

        let [_, game] = split_result;

        let mut higher_number_of_red = 0;
        let mut higher_number_of_green = 0;
        let mut higher_number_of_blue = 0;

        for turn in game.split(';') {
            let color_numbers = turn.split(", ");

            for color_number in color_numbers {
                let [number, color]: [String; 2] = color_number
                    .trim()
                    .split(" ")
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .try_into()
                    .unwrap_or_else(|v: Vec<String>| {
                        panic!("Expected 2 values but instead got {}", v.len())
                    });

                let number_value = number
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("Error converting color number"));

                if color == "red" && number_value > higher_number_of_red {
                    higher_number_of_red = number_value;
                    continue;
                }

                if color == "blue" && number_value > higher_number_of_blue {
                    higher_number_of_blue = number_value;
                    continue;
                }

                if color == "green" && number_value > higher_number_of_green {
                    higher_number_of_green = number_value;
                    continue;
                }
            }
        }

        sum += higher_number_of_red * higher_number_of_blue * higher_number_of_green;
    }

    println!("Day_2_Part_2: {}", sum);

    Ok(())
}
