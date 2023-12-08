use std::{fs::read_to_string};


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn sum_gear_ratios(lines: &[Vec<u8>]) -> u32 {
    let mut sum = 0;

    for pos in star_positions(&lines) {
        let numbers = adjacent_numbers(&lines, pos);
        if numbers.len() != 2 {
            continue;
        }

        let a = std::str::from_utf8(numbers[0])
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let b = std::str::from_utf8(numbers[1])
            .unwrap()
            .parse::<u32>()
            .unwrap();
        sum += a * b;
    }

    sum
}

fn adjacent_numbers(lines: &[Vec<u8>], star_position: (usize, usize)) -> Vec<&[u8]> {
    let (star_line, star_pos) = star_position;
    let mut numbers = Vec::new();

    // Find numbers above and below
    for line_idx in [star_line - 1, star_line + 1] {
        let line = &lines[line_idx];

        let mut char_pos = star_pos - 1;
        while char_pos <= star_pos + 1 {
            if !line[char_pos].is_ascii_digit() {
                char_pos += 1;
                continue;
            }

            let mut left = char_pos;
            if char_pos < star_pos {
                while left > 0 && line[left - 1].is_ascii_digit() {
                    left -= 1;
                }
            }
            let mut right = char_pos;
            while right < line.len() - 1 && line[right + 1].is_ascii_digit() {
                right += 1;
            }

            numbers.push(&line[left..=right]);

            char_pos = right + 1;
        }
    }

    // Find number left of star
    let line = &lines[star_line];
    if line[star_pos - 1].is_ascii_digit() {
        let mut left = star_pos - 1;
        while left > 0 && line[left - 1].is_ascii_digit() {
            left -= 1;
        }

        numbers.push(&line[left..=star_pos - 1]);
    }

    // Find number right of star
    if line[star_pos + 1].is_ascii_digit() {
        let mut right = star_pos + 1;
        while right < line.len() - 1 && line[right + 1].is_ascii_digit() {
            right += 1;
        }

        numbers.push(&line[star_pos + 1..=right]);
    }

    numbers
}

fn star_positions<'l>(lines: &'l [Vec<u8>]) -> impl Iterator<Item = (usize, usize)> + 'l {
    lines.iter().enumerate().flat_map(|(l, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, char)| **char == b'*')
            .map(move |(c, _)| (l, c))
    })
}

fn prepare_input(input: Vec<String>) -> Vec<Vec<u8>> {
    input.iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>()
}


fn main() {
    let input = read_lines("sample");
    let lines = prepare_input(input);

    println!("Sum of gear ratios = {}", sum_gear_ratios(&lines));
}



