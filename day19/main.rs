use std::{fs, str::FromStr, collections::HashMap};

#[derive(Debug)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct MaterialRequirement(Material, u32);

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
            "ore" => MaterialRequirement(Material::Ore, ore_amount),
            "clay" => MaterialRequirement(Material::Clay, ore_amount),
            "obsidian" => MaterialRequirement(Material::Obsidian, ore_amount),
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
    geode_robot: Vec<MaterialRequirement>,
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
        let mut geode_robot = vec![];

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
                "geode" => { geode_robot = requirements },
                _ => return Err(ParseBlueprintError)
            }
        }

        Ok(Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd)]
struct State {
    ore_robots: u32,
    ore: u32,
    clay_robots: u32,
    clay: u32,
    obsidian_robots: u32,
    obsidian: u32,
    geode_robots: u32,
    geodes_open: u32,

    pending_ore_robots: u32,
    pending_clay_robots: u32,
    pending_obsidian_robots: u32,
    pending_geode_robots: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.geodes_open != other.geodes_open {
            return self.geodes_open.cmp(&other.geodes_open);
        }

        if self.geode_robots != other.geode_robots {
            return self.geode_robots.cmp(&other.geode_robots);
        }

        if self.obsidian != other.obsidian {
            return self.obsidian.cmp(&other.obsidian);
        }

        if self.obsidian_robots != other.obsidian_robots {
            return self.obsidian_robots.cmp(&other.obsidian_robots);
        }

        if self.clay != other.clay {
            return self.clay.cmp(&other.clay);
        }

        if self.clay_robots != other.clay_robots {
            return self.clay_robots.cmp(&other.clay_robots);
        }

        if self.ore_robots != other.ore_robots {
            return self.ore_robots.cmp(&other.ore_robots);
        }

        self.ore.cmp(&other.ore)
    }
}


impl State {
    fn new(ore_robots: u32) -> Self {
        State {
            ore_robots,
            ore:                     0,
            clay_robots:             0,
            clay:                    0,
            obsidian_robots:         0,
            obsidian:                0,
            geode_robots:            0,
            geodes_open:             0,

            pending_ore_robots:      0,
            pending_clay_robots:     0,
            pending_obsidian_robots: 0,
            pending_geode_robots:    0,
        }
    }

    fn mine_material(&mut self) {
        self.ore         += self.ore_robots;
        self.clay        += self.clay_robots;
        self.obsidian    += self.obsidian_robots;
        self.geodes_open += self.geode_robots;
    }

    fn commit_manufactured_robots(&mut self) {
        self.ore_robots      += self.pending_ore_robots;
        self.clay_robots     += self.pending_clay_robots;
        self.obsidian_robots += self.pending_obsidian_robots;
        self.geode_robots    += self.pending_geode_robots;

        self.pending_ore_robots      = 0;
        self.pending_clay_robots     = 0;
        self.pending_obsidian_robots = 0;
        self.pending_geode_robots    = 0;
    }

    fn manufacture_robots(
        &mut self,
        requirements: &Vec<MaterialRequirement>,
    ) -> u32 {
        let mut total: u32 = u32::MAX;

        for requirement in requirements {
            let num = match requirement {
                MaterialRequirement(Material::Ore, v) =>
                    self.ore / v,
                MaterialRequirement(Material::Clay, v) =>
                    self.clay / v,
                MaterialRequirement(Material::Obsidian, v) =>
                    self.obsidian / v,
                MaterialRequirement(Material::Geode, _) =>
                    0,
            };

            total = total.min(num);
        }

        for requirement in requirements {
            match requirement {
                MaterialRequirement(Material::Ore, v) =>
                    self.ore -= total * v,
                MaterialRequirement(Material::Clay, v) =>
                    self.clay -= total * v,
                MaterialRequirement(Material::Obsidian, v) =>
                    self.obsidian -= total * v,
                MaterialRequirement(Material::Geode, _) =>
                    {},
            }
        }

        total
    }


    fn meets_requirements(&self, requirements: &Vec<MaterialRequirement>) -> bool {
        let mut total: u32 = u32::MAX;

        for requirement in requirements {
            let num = match requirement {
                MaterialRequirement(Material::Ore, v) =>
                    self.ore / v,
                MaterialRequirement(Material::Clay, v) =>
                    self.clay / v,
                MaterialRequirement(Material::Obsidian, v) =>
                    self.obsidian / v,
                MaterialRequirement(Material::Geode, _) =>
                    0,
            };

            total = total.min(num);
        }

        total > 0
    }
}

fn part1(blueprints: &Vec<Blueprint>) -> String {
    let state = State::new(1);
    let mut memo: HashMap<(State, u32), State> = HashMap::new();
    let mut max_cache: HashMap<u32, State> = HashMap::new();
    let result = simulate_blueprint(&blueprints[0], &state, 24, &mut memo, &mut max_cache);
    println!("{result:?}");

    String::from("")
}



fn part2(blueprints: &Vec<Blueprint>) -> String {
    

    String::from("")
}

fn simulate_blueprint(
    blueprint: &Blueprint,
    state: &State,
    duration: u32,
    memo: &mut HashMap<(State, u32), State>,
    max_cache: &mut HashMap<u32, State>,
) -> State {
    if duration == 0 {
        return state.clone();
    }

    let maybe_max = max_cache.get(&duration);

    if let Some(maybe_max) = maybe_max {
        if maybe_max > state {
            return maybe_max.clone();
        }
    }

    let maybe_memoed = memo.get(&(state.clone(), duration));

    if let Some(found_state) = maybe_memoed {
        // println!("Found: {found_state:?}\n");
        return found_state.clone();
    }

    let mut states: Vec<State> = vec![];

    // Build geode robots as soon as possible, it doesn't make sense to wait here
    if state.meets_requirements(&blueprint.geode_robot) {
        // Build geode robots
        let mut state1 = state.clone();
        state1.pending_geode_robots = state1.manufacture_robots(&blueprint.geode_robot);
        state1.mine_material();
        state1.commit_manufactured_robots();

        states.push(simulate_blueprint(blueprint, &state1, duration - 1, memo, max_cache));
    }


    if state.meets_requirements(&blueprint.obsidian_robot) {
        // Build obsidian robots
        let mut state1 = state.clone();
        state1.pending_obsidian_robots = state1.manufacture_robots(&blueprint.obsidian_robot);
        state1.mine_material();
        state1.commit_manufactured_robots();

        states.push(simulate_blueprint(blueprint, &state1, duration - 1, memo, max_cache));
    }

    if state.meets_requirements(&blueprint.clay_robot) {
        // Build obsidian robots
        let mut state1 = state.clone();
        state1.pending_clay_robots = state1.manufacture_robots(&blueprint.clay_robot);
        state1.mine_material();
        state1.commit_manufactured_robots();

        // Do nothing
        let mut state2 = state.clone();
        state2.mine_material();
        state2.commit_manufactured_robots();

        states.push(simulate_blueprint(blueprint, &state1, duration - 1, memo, max_cache));
        states.push(simulate_blueprint(blueprint, &state2, duration - 1, memo, max_cache));
    }

    if state.meets_requirements(&blueprint.ore_robot) {
        // Build obsidian robots
        let mut state1 = state.clone();
        state1.pending_ore_robots = state1.manufacture_robots(&blueprint.ore_robot);
        state1.mine_material();
        state1.commit_manufactured_robots();

        // Do nothing
        let mut state2 = state.clone();
        state2.mine_material();
        state2.commit_manufactured_robots();

        states.push(simulate_blueprint(blueprint, &state1, duration - 1, memo, max_cache));
        states.push(simulate_blueprint(blueprint, &state2, duration - 1, memo, max_cache));
    }
    
    if states.len() == 0 {
        let mut state1 = state.clone();
        state1.mine_material();
        state1.commit_manufactured_robots();
        let result = simulate_blueprint(blueprint, &state1, duration - 1, memo, max_cache);
        memo.insert((state.clone(), duration), result.clone());


        let best_existing_state = max_cache.get(&duration);
        match best_existing_state {
            Some(state) => {
                if &result > state {
                    max_cache.insert(duration, result.clone());
                }
            },
            None => {
                max_cache.insert(duration, result.clone());
            },
        }

        result
    } else {
        let result = states.iter().max().unwrap().clone();

        memo.insert((state.clone(), duration), result.clone());
        // println!("States {states:?} \n\nMax {result:?}\n");

        let best_existing_state = max_cache.get(&duration);
        match best_existing_state {
            Some(state) => {
                if &result > state {
                    max_cache.insert(duration, result.clone());
                }
            },
            None => {
                max_cache.insert(duration, result.clone());
            },
        }

        result
    }
}
