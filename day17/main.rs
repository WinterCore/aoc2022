use std::{fs, collections::{HashSet, HashMap}, char::CharTryFromError, thread::sleep, time::{Duration, Instant}, cmp};
use ncurses::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(i32, i32);


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

    fn is_colliding_with_rock(&self, target: &Self) -> bool {
        self.points
            .iter()
            .any(|p| target.is_colliding_with_point(p))
    }

    fn is_colliding_with_point(&self, p: &Point) -> bool {
        self
            .points
            .iter()
            .any(|tp| tp.0 == p.0 && tp.1 == p.1)
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
    rested: HashMap<i32, HashSet<Point>>,
    width: u32,
    height_reached: u32,
}

use std::sync::atomic::{AtomicUsize, Ordering};

static MS: AtomicUsize = AtomicUsize::new(0);


impl Chamber {
    fn move_rock_dir(&self, rock: &RockCluster, dir: &Direction) -> Option<RockCluster> {
        let new_rock = rock.move_by(dir.to_move_delta());
        
        // let is_colliding_with_rested_rocks = points.iter().any(|p| new_rock.is_colliding_with_point(p));
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

        let is_colliding_with_walls = new_rock.points.iter().any(|p| p.0 < 0 || (self.width as i32) <= p.0);
        let is_colliding_with_floor = new_rock.points.iter().any(|p| p.1 < 0);

        // println!("{:?}\n{:?}\n{:?}\n\n", rows, points, self.rested);

        // println!("\n");
        // println!("{:?} {:?} {:?} {:?}", new_rock, is_colliding_with_floor, is_colliding_with_walls, is_colliding_with_rested_rocks);
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
                .unwrap_or(0) as u32
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

        // println!("\n\n\n");

        loop {
            let jet = &gas_jets[*gas_jet_idx % gas_jets.len()];
            let dir = if down { &Direction::Down } else { jet };

            let moved_rock = self.move_rock_dir(&rock, dir);
            // println!("{:?}", instant.elapsed().as_micros());
            
            if render {
                draw(self, gas_jets, *gas_jet_idx % gas_jets.len(), &rock);
            }
            // println!("{:?} {:?}", dir, moved_rock.as_ref().unwrap_or(&rock));
            


            if down {
                match moved_rock {
                    Some(new_rock) => {
                        rock = new_rock;
                    },
                    None => {
                        // println!("Resting {:?}", rock);
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

            getch();
            // sleep(Duration::from_millis(100));
            // sleep(Duration::from_millis(200));
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
        (height - 1) + (max_y - min_y) - height / 2,
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
        for Point(x, y) in points.iter() {
            mvaddch((y * -1) + shift_y, x + shift_x, '#' as u32);
        }
    }


    // Draw falling rock
    for Point(x, y) in falling_rock.points.iter() {
        mvaddch((y * -1) + shift_y, x + shift_x, '@' as u32);
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
    let contents = fs::read_to_string("./exampleinput")
        .expect("File not found");

    let parsed: Vec<Direction> = parse(&contents);

    println!("Part 1: {}", part1(&parsed, true));
    println!("Part 2: {}", part2());
}

fn part1(gas_jets: &Vec<Direction>, render: bool) -> String {
    if render {
        initscr();
        use_default_colors();
        start_color();
        init_pair(1, COLOR_BLACK, COLOR_GREEN);
    }

    let instant = Instant::now();
    let rock_clusters = RockCluster::get_rock_clusters();
    let mut gas_jet_idx = 0;

    let mut chamber = Chamber {
        rested: HashMap::new(),
        width: 7,
        height_reached: 0,
    };

    for rock_num in 0..2022 {
        let raw_rock_cluster = rock_clusters[rock_num % rock_clusters.len()].clone();

        let rock_cluster = raw_rock_cluster
            .move_by(Point(2, chamber.height_reached as i32 + 3));

        chamber.simulate_rock_fall(
            gas_jets,
            &mut gas_jet_idx,
            &rock_cluster,
            render,
        );

        if rock_num % rock_clusters.len() == 0 && gas_jet_idx % gas_jets.len() == 0 {
            println!("{}", chamber.height_reached);
        }
    }

    MS.fetch_add(instant.elapsed().as_micros() as usize, Ordering::SeqCst);
    println!("{:?}", MS.load(Ordering::Relaxed) / 1000);

    if render {
        getch();
        endwin();
    }


    String::from(chamber.height_reached.to_string())
}


fn part2() -> String {
    String::from("")
}
