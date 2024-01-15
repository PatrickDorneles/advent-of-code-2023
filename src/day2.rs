use core::panic;
use std::io;

use crate::get_file_text;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

pub fn solve_day_2() -> io::Result<()> {
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

        let [game_title, game] = split_result;

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

        if higher_number_of_red <= MAX_RED
            && higher_number_of_green <= MAX_GREEN
            && higher_number_of_blue <= MAX_BLUE
        {
            let [_, game_num]: [String; 2] = game_title
                .split(" ")
                .map(String::from)
                .collect::<Vec<String>>()
                .try_into()
                .unwrap_or_else(|v: Vec<String>| {
                    panic!("Expected 2 values but instead got {}", v.len())
                });

            let game_num_as_number = game_num
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Could not convert game number"));

            sum += game_num_as_number;
        }
    }

    println!("Day_2: {}", sum);

    Ok(())
}
