use std::{fs, str::FromStr};

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    goede_robot: u32,
}

#[derive(Debug)]
struct ParseBlueprintError;

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(":").collect();
        let part1 = split.get(0).ok_or(ParseBlueprintError)?;
        let id: u32 = part1.trim_start_matches("Blueprint ")
            .parse().map_err(|_| ParseBlueprintError)?;

        let part2 = split.get(1).ok_or(ParseBlueprintError)?;
        let costs_str = part2.split(".").collect::<Vec<&str>>();
        let ore_robot: u32 = costs_str
            .get(0)
            .ok_or(ParseBlueprintError)?
            .trim_start()
            .trim_start_matches("Each ore robot costs ")
            .chars()
            .take_while(|x| x.is_digit(10))
            .collect::<String>()
            .parse()
            .map_err(|_| ParseBlueprintError)?;
        println!("fhaskldfhl");

        println!("{ore_robot:?}");

        Ok(Blueprint {
            id,
            ore_robot: 0,
            clay_robot: 0,
            obsidian_robot: 0,
            goede_robot: 0,
        })
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines()
        .map(|x| x.parse().expect("Failed to parse blueprint"))
        .collect()
}

fn main() {
    let contents = fs::read_to_string("./exampleinput")
        .expect("File not found");

    let blueprints = parse(&contents);

    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    String::from("")
}

fn part2() -> String {
    String::from("")
}
