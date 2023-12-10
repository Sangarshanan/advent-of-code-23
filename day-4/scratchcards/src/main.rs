use std::{fs::read_to_string};


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn parse_nos(cards: &str) -> Vec<usize> {
    let parsed_cards = cards.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    parsed_cards
}

fn main() {
    let input = read_lines("input");
    let mut total = 0;

    for line in input {
        // Get Card Number
        let (_, card) = line.split_once(": ").unwrap();
        // Winning and Losing Cards
        let (winning, mine) = card.split_once(" | ").unwrap();
        let winning_nos = parse_nos(winning);
        let my_nos = parse_nos(mine);
        let mut card_matches = 0;
        for number in my_nos.iter() {
            if winning_nos.contains(number) {
                card_matches += 1;
            }
        }
        if card_matches > 0 {
            total+= i32::pow(2, card_matches-1);
        }
    }
    println!("{:?}", total);
}
