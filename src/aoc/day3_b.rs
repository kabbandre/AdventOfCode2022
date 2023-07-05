use std::fs::read_to_string;

const UPPERCASE_OFFSET: i32 = 38;
const LOWERCASE_OFFSET: i32 = 96;

pub fn solution(group_size: usize) {
    let mut shared_letters: Vec<char> = vec![];
    let mut group: Vec<&str> = vec![];
    let contents = read_to_string("./assets/day3.txt").unwrap();

    for line in contents.lines() {
        group.push(line);
        if group.len() == group_size {
            let (&first_line, other_lines) = group.split_first().unwrap();
            first_line.chars().any(|letter| -> bool {
                let is_shared = other_lines.iter().all(|other_line| other_line.contains(letter));            
                if is_shared {
                    shared_letters.push(letter);
                }
                is_shared
            });
            group.clear();            
        }
    }

    println!("Day 3 Part 2 result - {}", shared_letters.iter().map(|&letter| letter as i32 - if letter.is_uppercase() { UPPERCASE_OFFSET } else { LOWERCASE_OFFSET }).sum::<i32>())
}