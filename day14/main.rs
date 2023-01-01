use std::{fs, collections::{HashSet, HashMap}};

type Path = Vec<Point>;

#[derive(Debug, Clone, Copy)]
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

fn main() {
    let contents = fs::read_to_string("./exampleinput")
        .expect("File not found");

    let parsed = parse(&contents);
    let data = CaveReservoir {
        sand_source: Point(500, 0),
        rock_paths: parsed,
    };

    println!("{:?}", data);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2());
}

fn part1(data: &CaveReservoir) -> String {
    let wall = RockWall::from_rock_paths(&data.rock_paths);

    println!("{:?}", wall);

    String::from("")
}

fn part2() -> String {
    String::from("")
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Range(i32, i32);

#[derive(Debug)]
struct RockWall {
    vertical: HashMap<i32, HashSet<Range>>,   // x: [Range(y_start, y_end)]
    horizontal: HashMap<i32, HashSet<Range>>, // y: [Range(x_start, x_end)]
}

impl RockWall {
    fn is_hitting(self: &Self, point: &Point) -> bool {
        let is_hitting_horizontal_walls = self
            .horizontal
            .get(point.)
            .any(|);
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

