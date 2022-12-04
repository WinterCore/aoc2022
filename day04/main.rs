use std::fs;


type SectionRange = (i32, i32);

type ElfPair = (SectionRange, SectionRange);

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed: Vec<ElfPair> = contents
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|xs| xs
             .split(",")
             .map(|x| x
                  .split("-")
                  .map(|z| z.parse::<i32>().unwrap())
                  .collect::<Vec<i32>>()
              )
             .filter(|x| x.len() == 2)
             .map(|x| (x[0], x[1]))
             .collect::<Vec<SectionRange>>()
         )
        .filter(|x| x.len() == 2)
        .map(|x| (x[0], x[1]))
        .collect();


    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn fully_overlap(pair: &ElfPair) -> bool {
    let ((a1, b1), (a2, b2)) = pair;

    (a1 <= a2 && b1 >= b2)
    || (a1 >= a2 && b1 <= b2)
}

fn overlap(pair: &ElfPair) -> bool {
    let ((a1, b1), (a2, b2)) = pair;

    ! ((b1 < a2) || (a1 > b2))
}

fn part1(input: &Vec<ElfPair>) -> String {
    let result = input.into_iter()
        .filter(|x| fully_overlap(x))
        .collect::<Vec<&ElfPair>>()
        .len();

    result.to_string()
}

fn part2(input: &Vec<ElfPair>) -> String {
    let result = input.into_iter()
        .filter(|x| overlap(x))
        .collect::<Vec<&ElfPair>>()
        .len();

    result.to_string()
}
