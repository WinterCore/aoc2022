use std::fs;


type SectionRange = (i32, i32);

type ElfPair = (SectionRange, SectionRange);

fn main() {
    let contents = fs::read_to_string("./exampleinput")
        .expect("File not found");

    let parsed: Vec<Vec<SectionRange>> = contents
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|xs| xs
             .split(",")
             .map(|x| x.split("-").map(|z| z.parse::<i32>().unwrap()).collect::<Vec<i32>>())
             .map(|x| x.iter().map(|y| (1i32, 2i32)).collect::<SectionRange>())
             .collect::<Vec<SectionRange>>()
         )
        .collect();


    println!("{:?}", parsed);

    /*
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
    */
}

fn part1() -> String {

    String::from("")
}
