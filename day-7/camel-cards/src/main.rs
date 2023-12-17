use std::{
    fs::read_to_string, 
    collections::HashMap
};
use itertools::Itertools;

/*
250370104
251735672
*/

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn sort_cards_in_vectors(cards: &Vec<char>) -> Vec<u32> {
    let mut considered_cards = Vec::new();
    let mut qty_of_cards = Vec::new();
    for card in cards {
        if let Some(index) = considered_cards.iter().position(|item| item == card) {
            qty_of_cards[index] += 1;
        } else {
            considered_cards.push(*card);
            qty_of_cards.push(1);
        }
    }
    qty_of_cards
}

fn main() {

    // Mappings
    let mut power_map = HashMap::from([
        ("A".to_string(), 12),
        ("K".to_string(), 11),
        ("Q".to_string(), 10),
        ("J".to_string(), 9),
        ("T".to_string(), 8),
    ]);
    for (index, number) in (2..=9).enumerate() {
        power_map.insert(number.to_string(), index);
    }
    let mut hand_type_map = HashMap::from([
        (0, Vec::new()),
        (1, Vec::new()),
        (2, Vec::new()),
        (3, Vec::new()),
        (4, Vec::new()),
        (5, Vec::new()),
        (6, Vec::new()),
    ]);

    let lines = read_lines("sample");

    for line in lines {
        let hand: Vec<_> = line.split(' ').collect();
        let mut cards: Vec<char> = hand[0].chars().collect();
        let _bid = hand[1].parse::<i32>().unwrap();

        // Sort by Hand Type
        let sorted_counter = sort_cards_in_vectors(&cards);
        let hand_type = match sorted_counter.len() {
            1 => 6, // Five of a kind
            2 => if sorted_counter.contains(&4) {
                5 // Four of a kind
            } else {
                4 // Full house
            },
            3 => if sorted_counter.contains(&3) {
                3 // Three of a kind,
            } else {
                2 // Two pair
            },
            4 => 1, // One pair
            _ => 0 // High card
        };
        
        hand_type_map.get_mut(&hand_type).map(|val| val.push(
            cards
        ));

    }
    for key in hand_type_map.keys().sorted() {
        let hand_type_count = hand_type_map[key].len();

        if hand_type_count > 0 {
            if hand_type_count > 1 {
                for cards in hand_type_map[key].clone().into_iter() {
                    println!("{:?}", cards);
                    let mut card_strength: usize = 0;
                    for (i, card) in cards.iter().enumerate() {
                        // Strength Based on Order
                        card_strength += power_map[&card.to_string()] * (i+1);
                    }
                    println!("{:?}", card_strength);
                }   
            }
        }
    }
}
