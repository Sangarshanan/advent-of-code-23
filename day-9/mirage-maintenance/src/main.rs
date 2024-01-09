use std::{
    fs::read_to_string, 
};


fn is_zero(buf: &Vec<i32>) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}

fn calculate_difference(value: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
     for i in 0..(value.len()-1) {
        result.push(value[i+1] - value[i]);
     }
     result
}

fn main() {

    let mut sum_part1 = i32::from(0);
    let mut sum_part2 = i32::from(0);

    for line in read_to_string("input").unwrap().lines() {
        let value = line.to_string();
         let history: Vec<i32> = value
                                .split(" ")
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect();

         let mut difference = history.clone();
         let mut reversed_difference: Vec<_> = history.iter().copied().rev().collect();

         while !is_zero(&difference) {
            sum_part1 += i32::from(difference[difference.len() -1]);
            difference = calculate_difference(difference);

            sum_part2 += i32::from(reversed_difference[reversed_difference.len() -1]);
            reversed_difference = calculate_difference(reversed_difference);
         }

    }
    println!("Part 1: {:?}", sum_part1);
    println!("Part 1: {:?}", sum_part2);

}
