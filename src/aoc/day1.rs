use std::{fs::read_to_string, cmp::max};

pub fn solution(top: usize) {
    let mut result = vec![0; top];
    let mut current = 0;
    let contents = read_to_string("./assets/day1.txt").unwrap();

    for line in contents.lines() {
        if line.is_empty() {
            let (min_value_index, _) = result.iter().enumerate().min_by_key(|(_, &index)| index).unwrap();
            result[min_value_index] = max(result[min_value_index], current);
            current = 0;
            continue;
        }
        let value = line.parse::<i32>().unwrap_or(0);
        current += value;
    }

    println!("Day 1 result - sum of top {} elves: {}", top, result.iter().sum::<i32>())
}