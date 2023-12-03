use std::{fs::read_to_string, collections::HashMap};


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn calibration_sum_part1(input: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in input {

        // Sum of digits
        let digits: Vec<_> = line.chars()
            .filter_map(|x| x.to_digit(10))
            .collect();
        sum += digits[0] * 10 + digits[digits.len() - 1];
    }
    return sum;
}

fn calibration_sum_part2(filelines: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in filelines {
        let first_num = first_or_last_number(line.as_str(), false);
        let last_num = first_or_last_number(line.as_str(), true);
        sum += first_num * 10 + last_num;
    }
    return sum;
}

fn first_or_last_number(input: &str, reverse: bool) -> i32 {
    let mut i = 0;

    match reverse {
        true => {
            for c in input.chars().rev() {
                if c.is_digit(10) {
                    return c.to_digit(10).unwrap() as i32;
                }
        
                match check_for_word(input, input.len() - i - 1) {
                    Some(digit) => { return digit; },
                    None => { i += 1; }
                }
            }
        }
        false => {
            for c in input.chars() {
                if c.is_digit(10) {
                    return c.to_digit(10).unwrap() as i32;
                }
        
                match check_for_word(input, i) {
                    Some(digit) => { return digit; },
                    None => { i += 1; }
                }
            }
        }
    }
    0
}

fn check_for_word(input: &str, i: usize) -> Option<i32> {
    let word_digits: HashMap<String, i32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);

    for word in word_digits.keys() {
        let substr = match input.get(i..i+word.len()).ok_or("Out of bounds") {
            Ok(str)=> str,
            _ => {continue;}
        };

        if substr == word {
            return word_digits.get(word).copied();
        }
    }

    None
}


fn main() {
    let input = read_lines("input");
    println!("{:?} {:?}", 
        calibration_sum_part1(input.clone()),
        calibration_sum_part2(input)
    );
}

