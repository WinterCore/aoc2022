use std::{fs, collections::{HashSet, HashMap}, cmp, hash::Hash};
use ncurses::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(i64, i64);


#[derive(Debug, Clone)]
struct RockCluster {
    points: Vec<Point>,
}

impl RockCluster {
    fn get_rock_clusters() -> Vec<Self> {
        vec![
            // Horizontal Line
            Self { points: vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0)] },
            // Plus sign
            Self { points: vec![Point(1, 0), Point(1, 1), Point(0, 1), Point(2, 1), Point(1, 2)] },
            // Reversed L
            Self { points: vec![Point(2, 0), Point(2, 1), Point(2, 2), Point(1, 0), Point(0, 0)] }, 
            // Vertical Line
            Self { points: vec![Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3)] },
            // Cube
            Self { points: vec![Point(1, 0), Point(1, 1), Point(0, 0), Point(0, 1)] },
        ]
    }

    fn move_by(&self, Point(dx, dy): Point) -> Self {
        let points = self.points
            .iter()
            .map(|Point(x, y)| Point(x + dx, y + dy))
            .collect();

        RockCluster { points }
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_move_delta(&self) -> Point {
        match self {
            Direction::Right => Point(1, 0),
            Direction::Down => Point(0, -1),
            Direction::Left => Point(-1, 0),
        }
    }
}

#[derive(Debug)]
struct Chamber {
    rested: HashMap<i64, HashSet<Point>>,
    width: u64,
    height_reached: u64,
}

impl Chamber {
    fn move_rock_dir(&self, rock: &RockCluster, dir: &Direction) -> Option<RockCluster> {
        let new_rock = rock.move_by(dir.to_move_delta());
        
        let mut is_colliding_with_rested_rocks = false;
        
        for point in new_rock.points.iter() {
            let row = self.rested.get(&point.1);

            match row {
                Some(set) => {
                    if set.contains(&point) {
                        is_colliding_with_rested_rocks = true;
                        break;
                    }
                },
                _ => (),
            }
        }

        let is_colliding_with_walls = new_rock.points.iter().any(|p| p.0 < 0 || (self.width as i64) <= p.0);
        let is_colliding_with_floor = new_rock.points.iter().any(|p| p.1 < 0);

        if is_colliding_with_floor || is_colliding_with_walls || is_colliding_with_rested_rocks {
            return None;
        }

        Some(new_rock)
    }

    fn rest_rock_cluster(&mut self, rock: RockCluster) {
        self.height_reached = self.height_reached.max(
            rock.points
                .iter()
                .map(|x| x.1 + 1)
                .max()
                .unwrap_or(0) as u64
        );

        rock
            .points
            .iter()
            .for_each(|p| {
                let set = self.rested.get_mut(&p.1);

                match set {
                    Some(data) => {
                        data.insert(*p);
                    },
                    None => {
                        self.rested.insert(p.1, HashSet::from_iter([*p]));
                    },
                };
            });

    }

    fn simulate_rock_fall(
        &mut self,
        gas_jets: &Vec<Direction>,
        gas_jet_idx: &mut usize,
        rock: &RockCluster,
        render: bool,
    ) -> RockCluster {
        let mut rock = rock.clone();

        let mut down = false;

        loop {
            let jet = &gas_jets[*gas_jet_idx % gas_jets.len()];
            let dir = if down { &Direction::Down } else { jet };

            let moved_rock = self.move_rock_dir(&rock, dir);
            
            if render {
                draw(self, gas_jets, *gas_jet_idx % gas_jets.len(), &rock);
            }

            if down {
                match moved_rock {
                    Some(new_rock) => {
                        rock = new_rock;
                    },
                    None => {
                        self.rest_rock_cluster(rock.clone());
                        break;
                    },
                }
            } else {
                *gas_jet_idx += 1;

                if let Some(new_rock) = moved_rock {
                    rock = new_rock;
                }
            }

            down = ! down;
        }

        rock
    }
}

fn parse(input: &str) -> Vec<Direction> {
    let list = input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _   => panic!("Invalid character {} at pos {}", c, i),
            }
        })
        .collect::<Vec<Direction>>();

    list
}

fn draw(
    chamber: &Chamber,
    gas_jets: &Vec<Direction>,
    gas_jet_idx: usize,
    falling_rock: &RockCluster,
) {
    clear();

    let height = LINES() - 1;
    let width = COLS();
    let chamber_draw_width = (chamber.width + 1) as i32;

    let min_y = chamber.rested.keys().min().unwrap_or(&0);
    let max_y = chamber.rested.keys().max().unwrap_or(&0);

    let shift_x = 1;
    // let raw_shift_y = falling_rock.points.iter().max_by_key(|x| x.1).unwrap().1 * -1 + height / 2;
    let shift_y = cmp::max(
        (height - 1) + (*max_y as i32 - *min_y as i32) - height / 2,
        height - 1,
    );

    // Draw walls
    for i in 0..height {
        mvaddch(i, 0, '|' as u32);
        mvaddch(i, chamber_draw_width, '|' as u32);
    }

    // Draw floor
    for i in 0..=chamber_draw_width {
        if i == 0 || i == chamber_draw_width {
            mvaddch(height, i, '+' as u32);
        } else {
            mvaddch(height, i, '-' as u32);
        }
    }

    // Draw rested rocks 
    for points in chamber.rested.values() {
        for &Point(x, y) in points.iter() {
            mvaddch((y as i32 * -1) + shift_y, x as i32 + shift_x, '#' as u32);
        }
    }


    // Draw falling rock
    for &Point(x, y) in falling_rock.points.iter() {
        mvaddch((y as i32 * -1) + shift_y, x as i32 + shift_x, '@' as u32);
    }


    let jets_per_row = width - chamber_draw_width - 5;
    let jets_left_padding = 2;
    let jets_top_padding = 2;

    let get_jet_char = |jet: &Direction| {
        match jet {
            Direction::Left => '<' as u32,
            Direction::Right => '>' as u32,
            _ => panic!("Unsupported jet"),
        }
    };

    // Draw reached height
    mvaddstr(0, chamber_draw_width + jets_left_padding, &format!("Reached height: {}", chamber.height_reached));

    // Draw Jets
    for (i, jet) in gas_jets.iter().enumerate() {

        let y = i as i32 / jets_per_row + jets_top_padding;
        let x = i as i32 % jets_per_row + jets_left_padding + chamber_draw_width;

        if i == gas_jet_idx {
            attron(COLOR_PAIR(1));
            mvaddch(y, x, get_jet_char(jet));
            attroff(COLOR_PAIR(1));
        } else {
            mvaddch(y, x, get_jet_char(jet));
        }
    }

    refresh();

    //sleep(Duration::from_millis(15));
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed: Vec<Direction> = parse(&contents);

    println!("Part 1: {}", part1(&parsed, false));
    println!("Part 2: {}", part2(&parsed, false));
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n % 2 == 0 && n > 2 {
        return false;
    }

    let s = (n as f64).sqrt();
    for a in (3..=(s as u64)).step_by(2) {
        if n % a == 0 {
            return false;
        }
    }

    true
}

fn part1(gas_jets: &Vec<Direction>, render: bool) -> String {
    let result = simulate(gas_jets, 2022, render);

    String::from(result.to_string())
}


fn part2(gas_jets: &Vec<Direction>, render: bool) -> String {
    let result = simulate(gas_jets, 1000000000000, render);

    String::from(result.to_string())
}


fn simulate(gas_jets: &Vec<Direction>, rocks_max: usize, render: bool) -> u64 {
    if render {
        initscr();
        use_default_colors();
        start_color();
        init_pair(1, COLOR_BLACK, COLOR_GREEN);
    }

    let rock_clusters = RockCluster::get_rock_clusters();
    let mut gas_jet_idx = 0;

    let mut chamber = Chamber {
        rested: HashMap::new(),
        width: 7,
        height_reached: 0,
    };

    let mut init_heights_hash = HashMap::<(usize, usize), (u64, usize)>::new();
    let mut cycle_hash = HashMap::<(usize, usize), (u64, usize)>::new();

    let mut rock_num = 0;
    while rock_num < rocks_max {
        let rock_idx = rock_num % rock_clusters.len();
        let jet_idx = gas_jet_idx % gas_jets.len();

        let existing = init_heights_hash.get(&(rock_idx, jet_idx));

        if let Some((height, _)) = existing {
            if is_prime(*height) && chamber.height_reached % height == 0 {
                let existing = cycle_hash.get(&(rock_idx, jet_idx));
                if let Some(data) = existing {
                    let cycle_num_rocks = rock_num - data.1;
                    let cycle_rock_height = chamber.height_reached - data.0;
                    let repeats = (rocks_max - rock_num) / cycle_num_rocks;
                    let height_diff = repeats as u64 * cycle_rock_height;

                    let mut new_rested: HashMap<i64, HashSet<Point>> = HashMap::new();
                    for (&key, points) in chamber.rested.iter() {
                        let new_points = points
                            .iter()
                            .map(|&Point(x, y)| Point(x, y + height_diff as i64))
                            .collect();
                        new_rested.insert((key as u64 + height_diff) as i64, new_points);
                    }

                    chamber.rested = new_rested;

                    rock_num += repeats * cycle_num_rocks;
                    chamber.height_reached += height_diff;
                } else {
                    cycle_hash.insert((rock_idx, jet_idx), (chamber.height_reached, rock_num));
                }
            }
        }

        let raw_rock_cluster = rock_clusters[rock_idx].clone();

        let rock_cluster = raw_rock_cluster
            .move_by(Point(2, chamber.height_reached as i64 + 3));

        chamber.simulate_rock_fall(
            gas_jets,
            &mut gas_jet_idx,
            &rock_cluster,
            render,
        );

        if let None = existing {
            init_heights_hash.insert(
                (rock_idx, jet_idx),
                (chamber.height_reached, rock_num),
            );
        }

        rock_num += 1;
    }

    if render {
        getch();
        endwin();
    }


    chamber.height_reached
}
