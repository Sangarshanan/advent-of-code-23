use std::{fs::read_to_string};
use itertools::Itertools;

fn extract_number(line: &str) -> Vec<i32> {
    return line
    .trim()
    .split(' ')
    .flat_map(str::parse::<i32>)
    .collect::<Vec<_>>();
}


fn find_ways_to_win(t: i64, d: i64) -> i32 {
    let mut ways_to_win = 0;
    for hold_time in 0..(t+1) {
        let travel_distance = hold_time * (t - hold_time);
        if travel_distance > d {
            ways_to_win+=1
        }
    }
    ways_to_win
}


fn main() {
    // Part 1
    let lines = read_to_string("input").unwrap();

    let (times, distances) = lines
        .split('\n')
        .map(|line| {
            extract_number(line)
        })
        .collect_tuple()
        .unwrap();

    let mut product_of_ways = 1;

    for race_no in 0..times.len() {
        let t = times[race_no];
        let d = distances[race_no];
        product_of_ways *= find_ways_to_win(
            t as i64,
            d as i64
        );
    }
    println!("{:?}", product_of_ways);

    // Part 2
    let time_c : i64 = times
                        .iter()
                        .join("")
                        .trim()
                        .parse()
                        .unwrap();
    let distance_c : i64 = distances
                        .iter()
                        .join("")
                        .trim()
                        .parse()
                        .unwrap();

    println!("{:?}", find_ways_to_win(time_c, distance_c));

}
