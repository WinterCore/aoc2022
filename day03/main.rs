use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::process;

fn get_priority(c: &char) -> i32 {
    if c.is_ascii_uppercase() {
        return (*c as i32) - ('A' as i32) + 27;
    }

    (*c as i32) - ('a' as i32) + 1
}

fn find_common(strs: Vec<&[char]>) -> Option<char> {
    if strs.len() < 2 {
        return None;
    }

    let first = strs.first().unwrap();
    let sets: Vec<HashSet<char>> = strs.iter().skip(1).map(|s| HashSet::from_iter(s.iter().cloned())).collect();

    let common = first.into_iter().find(|c| sets.iter().all(|s| s.contains(c)));
        
    match common {
        Some(c) => Some(c.clone()),
        None => None
    }
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed: Vec<Vec<char>> = contents
        .split("\n")
        .filter(|x| x.len() > 0 && x.len() % 2 == 0)
        .map(|x| x.chars().collect())
        .collect();


    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn part1(input: &Vec<Vec<char>>) -> String {
    let result: i32 = input.iter()
        .map(|xs| find_common(vec![&xs[0..(xs.len() / 2)], &xs[(xs.len() / 2)..]]))
        .map(|c| get_priority(&c.unwrap()))
        .sum();

    result.to_string()
}

fn part2(input: &Vec<Vec<char>>) -> String {
    let result: i32 = input
        .chunks(3)
        .map(|x| find_common(x.iter().map(|x| x.as_slice()).collect()))
        .map(|c| get_priority(&c.unwrap()))
        .sum();

    result.to_string()
}
