use std::{fs, str::FromStr};

#[derive(Debug)]
enum MaterialRequirement {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
    Geode(u32),
}

#[derive(Debug)]
struct ParseMaterialRequirementError;

impl FromStr for MaterialRequirement {
    type Err = ParseMaterialRequirementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(" ").collect();
        let ore_amount: u32 = parts
            .get(0)
            .ok_or(ParseMaterialRequirementError)
            .and_then(|x| x.parse::<u32>().map_err(|_| ParseMaterialRequirementError))?;
        let ore_name = parts
            .get(1)
            .ok_or(ParseMaterialRequirementError)?;

        let result = match *ore_name {
            "ore" => MaterialRequirement::Ore(ore_amount),
            "clay" => MaterialRequirement::Clay(ore_amount),
            "obsidian" => MaterialRequirement::Obsidian(ore_amount),
            "geode" => MaterialRequirement::Geode(ore_amount),
            _ => return Err(ParseMaterialRequirementError),
        };

        Ok(result)
    }
}


#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot: Vec<MaterialRequirement>,
    clay_robot: Vec<MaterialRequirement>,
    obsidian_robot: Vec<MaterialRequirement>,
    goede_robot: Vec<MaterialRequirement>,
}

impl Blueprint {
    fn parse_ore_name(data: &str) -> Option<String> {
        let ore_name = data.split(" ").nth(1)?;
        Some(ore_name.to_owned())
    }

    fn parse_ore_requirements(data: &str) -> Option<Vec<MaterialRequirement>> {
        let parts = data.split("and");

        let mut requirements: Vec<MaterialRequirement> = vec![];

        for part in parts {
            requirements.push(part.trim().trim_end_matches(".").parse().ok()?);
        }
        
        Some(requirements)
    }
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

        let mut ore_robot = vec![];
        let mut clay_robot = vec![];
        let mut obsidian_robot = vec![];
        let mut goede_robot = vec![];

        let part2 = split.get(1).ok_or(ParseBlueprintError)?;

        for orestr in part2.trim().trim_start_matches("Each").split("Each") {
            let parts: Vec<&str> = orestr.split("costs").collect();
            let name_part = parts.get(0).ok_or(ParseBlueprintError)?;
            let requirements_part = parts.get(1).ok_or(ParseBlueprintError)?;
            let name = Blueprint::parse_ore_name(&name_part)
                .ok_or(ParseBlueprintError)?;
            let requirements = Blueprint::parse_ore_requirements(&requirements_part)
                .ok_or(ParseBlueprintError)?;

            match name.as_str() {
                "ore" => { ore_robot = requirements },
                "clay" => { clay_robot = requirements },
                "obsidian" => { obsidian_robot = requirements },
                "geode" => { goede_robot = requirements },
                _ => return Err(ParseBlueprintError)
            }
        }

        Ok(Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            goede_robot,
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

    println!("Part 1: {}", part1(&blueprints));
    println!("Part 2: {}", part2(&blueprints));
}

fn part1(blueprints: &Vec<Blueprint>) -> String {
    String::from("")
}

fn part2(blueprints: &Vec<Blueprint>) -> String {
    String::from("")
}
