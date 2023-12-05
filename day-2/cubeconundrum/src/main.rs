use std::{fs::read_to_string};
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn cube_conundrum(input: Vec<String>) {

    let mut value_part1 = 0;
    let mut value_part2 = 0;

    for (idx, line) in input.iter().enumerate() {
        let game_id = idx + 1;
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        let pattern = Regex::new(r"(\d+) (\w+)").unwrap();
        
        for captures in pattern.captures_iter(line) {
            let number: u32 = captures[1].parse().unwrap();
            let color = captures[2].to_string();

            if color == "blue" {
                blue = blue.max(number);
            }
            if color == "red" {
                red = red.max(number);
            }
            if color == "green" {
                green = green.max(number);
            }

        }
        if red <= 12 && green <= 13 && blue <= 14 {
            value_part1 += game_id;
        }
        value_part2 += green * red * blue;

    }
    println!("Part 1: {:?}", value_part1);
    println!("Part 2: {:?}", value_part2);
}

fn main() {
    let input = read_lines("input");
    cube_conundrum(input)
}

