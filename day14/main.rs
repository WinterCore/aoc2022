use core::time;
use std::{fs, collections::{HashSet, HashMap}, cmp, thread};
use ncurses::*;

type Path = Vec<Point>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point (i32, i32);

// I couldn't find a better name
#[derive(Debug)]
struct CaveReservoir {
    sand_source: Point,
    rock_paths: Vec<Path>,
}

fn parse(input: &str) -> Vec<Path> {
    input.trim()
        .lines()
        .map(|l| l
             .trim()
             .split(" -> ")
             .map(|p| {
                 let parts = p.split(",").collect::<Vec<&str>>();
                Point(parts[0].parse().unwrap(), parts[1].parse().unwrap())
             }).collect::<Vec<Point>>()
         ).collect()
}

struct LineTraversalParams {
    start: Point,
    end: Point,
    dx: i32,
    dy: i32,
}

fn get_line_traversal_params(a: &Point, b: &Point) -> LineTraversalParams {
    if a.0 == b.0 && a.1 < b.1 {
        return LineTraversalParams { start: *a, end: *b, dx: 0, dy: 1 };
    }

    if a.0 == b.0 && a.1 > b.1 {
        return LineTraversalParams { start: *b, end: *a, dx: 0, dy: 1 };
    }

    if a.1 == b.1 && a.0 < b.0 {
        return LineTraversalParams { start: *a, end: *b, dx: 1, dy: 0 };
    }

    if a.1 == b.1 && a.0 > b.0 {
        return LineTraversalParams { start: *b, end: *a, dx: 1, dy: 0 };
    }

    panic!("Unsupported traversal a: {:?} - b: {:?}", a, b);
}

struct WallsMinMax {
    min: Point,
    max: Point,
}

fn get_walls_min_max(paths: &Vec<Path>) -> WallsMinMax {
    let rock_paths_iter = paths.iter().flatten();

    let min_x = rock_paths_iter.clone().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_x = rock_paths_iter.clone().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = rock_paths_iter.clone().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_y = rock_paths_iter.clone().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    

    WallsMinMax {
        min: Point(min_x, min_y),
        max: Point(max_x, max_y),
    }
}

fn draw(
    data: &CaveReservoir,
    rested: &HashSet<Point>,
    current: &Point,
    wait: bool,
) {
    let CaveReservoir {
        sand_source,
        rock_paths,
    } = data;
    let walls_mm = get_walls_min_max(rock_paths);

    let min_x = cmp::min(walls_mm.min.0, sand_source.0);
    let max_x = cmp::max(walls_mm.max.0, sand_source.0);
    let min_y = cmp::min(walls_mm.min.1, sand_source.1) - 2;
    let max_y = cmp::max(walls_mm.max.1, sand_source.1) + 4;
    let shift_x = if min_x > 0 { -min_x } else { min_x.abs() };
    let shift_y = if min_y > 0 { -min_y } else { min_y.abs() };

    let h = (max_y - min_y).abs();
    let w = cmp::max((max_x - min_x).abs(), 30);
    let win = newwin(h, w, 0, 0);

    let term_h = 50;
    let max_y_rested = rested.iter().max_by(|a, b| a.1.cmp(&b.1)).map_or(0, |x| x.1);
    let camera_y = cmp::max(0, max_y_rested + shift_y - (term_h / 2)) * -1;


    wclear(win);

    // Draw the sand faucet
    mvwaddch(win, sand_source.1 + shift_y + camera_y, sand_source.0 + shift_x, '+' as u32);

    // Draw walls
    for path in rock_paths {
        let mut p = path[0];
        for i in 1..path.len() {
            let c = path[i];
            let LineTraversalParams {
                mut start,
                end,
                dx,
                dy,
            } = get_line_traversal_params(&p, &c);

            while start.0 <= end.0 && start.1 <= end.1 {
                mvwaddch(win, start.1 + shift_y + camera_y, start.0 + shift_x, '#' as u32);

                start.0 += dx;
                start.1 += dy;
            }

            p = c;
        }
    }


    // Draw rested sand particles
    for Point(x, y) in rested.iter() {
        mvwaddch(win, y + shift_y + camera_y, x + shift_x, 'o' as u32);
    }


    // Draw sand particle being simulated
    wattron(win, COLOR_PAIR(1));
    mvwaddch(win, current.1 + shift_y + camera_y, current.0 + shift_x, 'o' as u32);
    wattroff(win, COLOR_PAIR(1));


    if wait {
        mvwaddstr(win, max_y + shift_y - 1, 0, "Press any key to continue...");
    }

    wrefresh(win);

    if wait {
        wgetch(win);
    }

    // Only wait if the simulated sand particle is in the viewport
    if current.1 + shift_y + camera_y > 0 {
        thread::sleep(time::Duration::from_millis(4));
    }
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed = parse(&contents);
    let data = CaveReservoir {
        sand_source: Point(500, 0),
        rock_paths: parsed,
    };

    println!("Part 1: {}", part1(&data, false));
    println!("Part 2: {}", part2(&data, false));
}

fn part1(data: &CaveReservoir, render: bool) -> String {
    let wall = RockWall::from_rock_paths(&data.rock_paths);
    let lowest_y = *wall.horizontal.keys().max().unwrap(); // Find highest Y key
    let mut rested = HashSet::<Point>::new();

    let mut sand_particle = data.sand_source;

    if render {
        initscr();
        use_default_colors();
        start_color();
        init_pair(1, COLOR_RED, -1);
    }

    // Keep simulating until a sand particle goes below the lowest y point (the floor)
    while sand_particle.1 < lowest_y {
        let Point(x, y) = sand_particle;

        let possible_moves = [
            Point(x, y + 1),
            Point(x - 1, y + 1),
            Point(x + 1, y + 1),
        ];

        let next_pos = possible_moves
            .iter()
            .find(|p| ! wall.is_colliding(p) && ! rested.contains(p));

        match next_pos {
            Some(p) => sand_particle = *p,
            None    => {
                rested.insert(sand_particle);
                sand_particle = data.sand_source;
            },
        }

        if render {
            draw(data, &rested, &sand_particle, false);
        }
    }

    if render {
        draw(data, &rested, &sand_particle, true);
        endwin();
    }

    String::from(rested.len().to_string())
}

fn part2(orig_data: &CaveReservoir, render: bool) -> String {
    let walls_mm = get_walls_min_max(&orig_data.rock_paths);
    let mut full_rock_paths: Vec<Path> = orig_data.rock_paths.clone();

    // IMPORTANT NOTE: MAKE SURE YOU CHANGE THE x VALUES HERE TO SOMETHING SMALLER THAN i32::MIN
    // and i32::MAX otherwise the draw function will crash the program
    let floor: Path = vec![Point(i32::MIN, walls_mm.max.1 + 2), Point(i32::MAX, walls_mm.max.1 + 2)];

    full_rock_paths.push(floor);
    let data = &CaveReservoir { sand_source: orig_data.sand_source, rock_paths: full_rock_paths };

    let wall = RockWall::from_rock_paths(&data.rock_paths);
    let mut rested = HashSet::<Point>::new();

    let mut sand_particle = data.sand_source;

    if render {
        initscr();
        use_default_colors();
        start_color();
        init_pair(1, COLOR_RED, -1);
    }


    loop {
        let Point(x, y) = sand_particle;

        let possible_moves = [
            Point(x, y + 1),
            Point(x - 1, y + 1),
            Point(x + 1, y + 1),
        ];

        let next_pos = possible_moves
            .iter()
            .find(|p| ! wall.is_colliding(p) && ! rested.contains(p));

        match next_pos {
            Some(p) => sand_particle = *p,
            None    => {
                rested.insert(sand_particle);

                // Keep simulating until a sand particle blocks the faucet
                if sand_particle == data.sand_source {
                    break;
                }

                sand_particle = data.sand_source;
            },
        }

        if render {
            draw(data, &rested, &sand_particle, false);
        }
    }

    if render {
        draw(data, &rested, &sand_particle, true);
        endwin();
    }

    String::from(rested.len().to_string())
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Range(i32, i32);

#[derive(Debug)]
struct RockWall {
    vertical: HashMap<i32, HashSet<Range>>,   // x: [Range(y_start, y_end)]
    horizontal: HashMap<i32, HashSet<Range>>, // y: [Range(x_start, x_end)]
}

impl RockWall {
    fn is_colliding(self: &Self, point: &Point) -> bool {
        let Point(x, y) = point;

        let is_within = |a, v, b| -> bool {
            let min = cmp::min(a, b);
            let max = cmp::max(a, b);

            min <= v && v <= max
        };

        let is_hitting_horizontal_walls = self
            .horizontal
            .get(&y)
            .map_or(
                false,
                |v| v.iter().any(|Range(sx, ex)| is_within(sx, x, ex))
            );

        let is_hitting_vertical_walls = self
            .vertical
            .get(&x)
            .map_or(
                false,
                |v| v.iter().any(|Range(sy, ey)| is_within(sy, y, ey))
            );

        is_hitting_vertical_walls || is_hitting_horizontal_walls
    }

    fn from_rock_paths(rock_paths: &Vec<Path>) -> Self {
        let mut horizontal = HashMap::<i32, HashSet<Range>>::new();
        let mut vertical = HashMap::<i32, HashSet<Range>>::new();

        for path in rock_paths {
            for i in 0..(path.len() - 1) {
                let (curr@Point(cx, cy), next@Point(nx, ny)) = (path[i], path[i + 1]);

                // TODO: Performance improvement: connect overlapping ranges
                if cx == nx {
                    let range = if cy > ny { Range(ny, cy) } else { Range(cy, ny) };
                    vertical
                        .entry(cx)
                        .and_modify(|rs| { rs.insert(range.clone()); })
                        .or_insert(HashSet::from([range.clone()]));
                } else if cy == ny {
                    let range = if cx > cy { Range(nx, cx) } else { Range(cx, nx) };
                    horizontal
                        .entry(cy)
                        .and_modify(|rs| { rs.insert(range.clone()); })
                        .or_insert(HashSet::from([range.clone()]));
                } else {
                    panic!("Diagonal paths aren't supported {:?} -> {:?}", curr, next);
                }
            }
        }

        RockWall { vertical, horizontal }
    }
}

