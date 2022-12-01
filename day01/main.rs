use std::fs;

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed: Vec<Vec<i32>> = contents
        .split("\n\n")
        .map(|x| x.split("\n").filter(|s| s.len() > 0).map(|y| { y.parse::<i32>().unwrap() }).collect())
        .collect();

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}


fn part1(parsed: &Vec<Vec<i32>>) -> String {

    let elf_calories: Vec<i32> = parsed.iter().map(|elf| elf.iter().sum()).collect();

    let max = elf_calories.iter().max().unwrap();

    String::from(max.to_string())
}

fn part2(parsed: &Vec<Vec<i32>>) -> String {

    let mut elf_calories: Vec<i32> = parsed.iter().map(|elf| elf.iter().sum()).collect();
    
    elf_calories.sort_unstable();

    let top_3_total = &elf_calories[(elf_calories.len() - 3)..];
    let result: i32 = top_3_total.iter().sum();

    String::from(result.to_string())
}
