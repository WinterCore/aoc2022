use std::{fs, collections::HashSet, char::CharTryFromError, thread::sleep, time::Duration, cmp};
use ncurses::*;

#[derive(Debug, Clone, Copy)]
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
            Self { points: vec![Point(2, 0), Point(2, 1), Point(2, 2), Point(1, 2), Point(0, 2)] }, 
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
    Top,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_move_delta(&self) -> Point {
        match self {
            Direction::Top => Point(0, -1),
            Direction::Right => Point(1, 0),
            Direction::Down => Point(0, 1),
            Direction::Left => Point(-1, 0),
        }
    }
}

#[derive(Debug)]
struct RegionChamber {
    // https://gamedev.stackexchange.com/a/76209
    zone_rock_clusters: Vec<HashSet<usize>>,
    zone_size: u32,
    width: u32,
    max_height: u32,

    rested_rock_clusters: Vec<RockCluster>,
}

impl RegionChamber {
    fn get_point_zone(&self, Point(x, y): &Point) -> usize {
        let zones_per_width = (self.width as f32 / self.zone_size as f32).ceil() as usize;
        let zone = (y * zones_per_width as i32) + (x / self.zone_size as i32);

        zone as usize
    }

    fn is_colliding_with_zone(&self, rock: &RockCluster, zone: usize) -> bool {
        if let None = self.zone_rock_clusters.get(zone) {
            return false;
        }

        self.zone_rock_clusters[zone]
            .iter()
            .filter_map(|&i| self.rested_rock_clusters.get(i))
            .any(|rc| rc.is_colliding_with_rock(rock))
    }

    fn is_rock_colliding(&self, rock: &RockCluster) -> bool {
        let is_colliding_with_floor = rock.points
            .iter()
            .any(|p| p.1 >= self.max_height as i32);

        let is_colliding_with_walls = rock.points
            .iter()
            .any(|p| p.0 < 0 || (self.width as i32) <= p.0);

        // Walls collision
        if is_colliding_with_walls || is_colliding_with_floor {
            return true;
        }

        let covered_zones: HashSet<usize> = rock.points
          .iter()
          .fold(HashSet::new(), |mut hs, p|  {
              hs.insert(self.get_point_zone(p));
              hs
          });

        // Optimize by only checking for zones that exist in provided direction
        covered_zones
            .iter()
            .any(|&z| self.is_colliding_with_zone(rock, z))
    }

    fn rest_rock_cluster(&mut self, rock: RockCluster) {
        let idx = self.rested_rock_clusters.len(); 

        rock
            .points
            .iter()
            .for_each(|p| {
                let zone = self.get_point_zone(p);
                
                if let Some(clusters) = self.zone_rock_clusters.get_mut(zone) {
                    clusters.insert(idx);
                }
            });

        self.rested_rock_clusters.push(rock);
    }

    fn move_rock_dir(&self, rock: &RockCluster, dir: &Direction) -> Option<RockCluster> {
        let new_rock = rock.move_by(dir.to_move_delta());
        let is_colliding = self.is_rock_colliding(&new_rock);

        if is_colliding {
            return None;
        }

        Some(new_rock)
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
            
            if render {
                draw(self, gas_jets, *gas_jet_idx, &rock);
            }
            // println!("{:?} {:?}", dir, moved_rock.as_ref().unwrap_or(&rock));


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


    /*
    let mut grouped: Vec<GasJet> = vec![];

    for curr in list.into_iter() {
        let last = grouped.last_mut();

        match last {
            Some(last) => {
                if std::mem::discriminant(last) == std::mem::discriminant(&curr) {
                    *last = last.add(curr.val());
                } else {
                    grouped.push(curr);
                }
            },
            None => {
                grouped.push(curr);
            }
        }
    }

    grouped
    */

    list
}

fn draw(
    chamber: &RegionChamber,
    gas_jets: &Vec<Direction>,
    gas_jet_idx: usize,
    falling_rock: &RockCluster,
) {
    clear();

    let height = LINES() - 1;
    let width = COLS();
    let chamber_draw_width = (chamber.width + 2) as i32;

    let shift_x = 2;
    let raw_shift_y = falling_rock.points.iter().max_by_key(|x| x.1).unwrap().1 * -1 + height / 2;
    let shift_y = cmp::max(
        raw_shift_y,
        (chamber.max_height as i32 - height) * -1,
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
    for rested_rock in chamber.rested_rock_clusters.iter() {
        for Point(x, y) in rested_rock.points.iter() {
            mvaddch(y + shift_y, x + shift_x, '#' as u32);
        }
    }


    // Draw falling rock
    for Point(x, y) in falling_rock.points.iter() {
        mvaddch(y + shift_y, x + shift_x, '@' as u32);
    }


    let jets_per_row = width - chamber_draw_width - 5;
    let jets_left_padding = 2;

    let get_jet_char = |jet: &Direction| {
        match jet {
            Direction::Left => '<' as u32,
            Direction::Right => '>' as u32,
            _ => panic!("Unsupported jet"),
        }
    };

    // Draw Jets
    for (i, jet) in gas_jets.iter().enumerate() {

        let y = i as i32 / jets_per_row;
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

    sleep(Duration::from_millis(50));
}

fn main() {
    let contents = fs::read_to_string("./exampleinput")
        .expect("File not found");

    let parsed: Vec<Direction> = parse(&contents);

    println!("Part 1: {}", part1(&parsed, true));
    println!("Part 2: {}", part2());
}

fn part1(gas_jets: &Vec<Direction>, render: bool) -> String {
    let rock_clusters = RockCluster::get_rock_clusters();
    let width = 7;
    let max_height = 10000;
    let zone_size = 4;
    let zone_count: usize = (width as f32 / zone_size as f32).ceil() as usize * max_height as usize;
    // println!("Zone count: {}", zone_count);

    if render {
        initscr();
        use_default_colors();
        start_color();
        init_pair(1, COLOR_RED, -1);
    }

    let mut lowest_y = max_height as i32;
    let mut chamber = RegionChamber {
        rested_rock_clusters: Vec::new(),
        zone_rock_clusters: vec![HashSet::new(); zone_count],
        zone_size,
        width,
        max_height,
    };

    let mut gas_jet_idx = 0;

    for rock_num in 0..2022 {
        let raw_rock_cluster = rock_clusters[rock_num % rock_clusters.len()].clone();
        let raw_rock_cluster_bottom = raw_rock_cluster
            .points
            .iter()
            .max_by_key(|x| x.1)
            .unwrap()
            .1;

        let rock_cluster = raw_rock_cluster
            .move_by(Point(2, lowest_y - (4 + raw_rock_cluster_bottom)));

        let rested_rock = chamber.simulate_rock_fall(
            gas_jets,
            &mut gas_jet_idx,
            &rock_cluster,
            render,
        );

        rested_rock.points.iter().for_each(|p| {
            lowest_y = lowest_y.min(p.1);
        });
    }

    if render {
        getch();
        endwin();
    }

    // println!("{:?}", chamber);
    println!("{:?}", max_height as i32 - lowest_y);
    String::from("")
}


fn part2() -> String {
    String::from("")
}
