use std::fs::read_to_string;

const UPPERCASE_OFFSET: i32 = 38;
const LOWERCASE_OFFSET: i32 = 96;

pub fn solution() {
    let mut shared_letters: Vec<char> = vec![];
    let contents = read_to_string("./assets/day3.txt").unwrap();

    for line in contents.lines() {
        let middle_index = line.len() / 2;
        let (first, second) = line.split_at(middle_index);
        first.chars().any(|letter| -> bool {
            if second.contains(letter) {
                shared_letters.push(letter);
                return true;
            }
            false
        });
    }

    println!("Day 3 Part 1 result - {}", shared_letters.iter().map(|&letter| letter as i32 - if letter.is_uppercase() { UPPERCASE_OFFSET } else { LOWERCASE_OFFSET }).sum::<i32>())
}