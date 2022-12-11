use std::{fs, collections::HashMap, cmp::max};


fn parse(input: &str) -> Vec<char> {
    input.chars().collect::<Vec<char>>()
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");
    let trimmed = contents.trim();

    let parsed = parse(trimmed);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn find_unique_chars_idx(data: &Vec<char>, n: u32) -> Option<u32> {
    let mut last_seen: HashMap<char, u32> = HashMap::new();

    let mut invalid_until = 0u32;

    for (i, c) in data.iter().enumerate() {
        let curr_last_seen = last_seen.get(c);

        if let Some(lp) = curr_last_seen {
            invalid_until = max(invalid_until, *lp);
        }

        last_seen.insert(*c, i as u32);

        if i as u32 >= invalid_until + n {
            return Some((i + 1) as u32);
        }
    }

    None
}

fn part1(data: &Vec<char>) -> String {
    let result = find_unique_chars_idx(data, 4);

    if let Some(idx) = result { String::from(idx.to_string()) } else { String::from("Not found") }
}

fn part2(data: &Vec<char>) -> String {
    let result = find_unique_chars_idx(data, 14);

    if let Some(idx) = result { String::from(idx.to_string()) } else { String::from("Not found") }
}

