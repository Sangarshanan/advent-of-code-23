use regex::Regex;
use std::{
    fs::read_to_string,
    collections::HashMap,
};

/*
Part 1: 15517
Part 2: 14935034899483
*/

fn parse_nodes(lines: Vec<&str>) -> HashMap<String, (String, String)> {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for (_index, line) in lines.iter().enumerate() {
        let re = Regex::new(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)").unwrap();

        if let Some(captures) = re.captures(line) {
            let key = captures[1].to_string();
            let value1 = captures[2].to_string();
            let value2 = captures[3].to_string();

            nodes.insert(key, (value1, value2));

        }
    }
    nodes
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn find_lcm_of_array(arr: &[u64]) -> u64 {
    arr.iter().fold(1, |acc, &num| lcm(acc, num))
}

fn main() {
    let file = read_to_string("input2").unwrap();
    let lines: Vec<_> =  file.split('\n').collect();

    let nodes = parse_nodes(lines.clone());
    let instruction =  lines[0];

    // Part 1

    let mut current = "AAA";
    let mut step = 0;

    while current != "ZZZ" {
        for ins in instruction.chars().collect::<Vec<_>>() {
            if ins == 'L' {
                current = &nodes[current].0;
            } else {
                current = &nodes[current].1;
            }
            step+=1;
        }
    }
    println!("Part 1:{:?}", step);


    // Part 2

    let keys_ending_with_a: Vec<&String> = nodes
                                            .keys()
                                            .filter(
                                                |&k| k.ends_with('A')
                                            ).collect();

    // Traversing all keys till the end will give us a circle
    // Different keys have circles of different length
    // Find where the circles converge on a common key ending with Z

    let mut total_cycle_len: Vec<u64> = Vec::new();
    for mut current in keys_ending_with_a {
        let mut cycle_len = 0;
        for ins in instruction.chars().cycle() {
            if ins == 'L' {
                current = &nodes[current].0;
            } else {
                current = &nodes[current].1;
            }
            cycle_len+=1;
            if current.ends_with("Z") {
                break
            }
        }
        total_cycle_len.push(cycle_len);
    }
    println!("Part 2: {:?}",find_lcm_of_array(&total_cycle_len));

}
