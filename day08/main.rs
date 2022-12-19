use std::{fs, i32, u32, cmp::max};

type Point = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

type DirVisionMap = Vec<Vec<u32>>;

#[derive(Debug)]
struct VisionMap {
    top: Vec<Vec<u32>>,
    bottom: Vec<Vec<u32>>,
    left: Vec<Vec<u32>>,
    right: Vec<Vec<u32>>,
}

#[derive(Debug)]
struct VisionScores {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|xs| xs
             .chars()
             .map(|c| c.to_digit(10).expect("Invalid digit"))
             .collect::<Vec<u32>>())
        .collect()
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed = parse(&contents);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}


fn part1(grid: &Vec<Vec<u32>>) -> String {
    let vision_map = get_vision_map(grid, false);

    let (xmax, ymax) = get_grid_limits(grid);
    let mut visible = 0;

    for y in 0..ymax {
        for x in 0..xmax {
            let scores = get_vision_scores(&vision_map, &(x, y), xmax, ymax);

            if scores.left == x as u32
               || scores.top == y as u32
               || scores.right == ((xmax - 1) - x) as u32
               || scores.bottom == ((ymax - 1) - y) as u32 {

                visible += 1;
            }
        }
    }

    String::from(visible.to_string())
}

fn part2(grid: &Vec<Vec<u32>>) -> String {
    let vision_map = get_vision_map(grid, true);

    let (xmax, ymax) = get_grid_limits(grid);
    let mut max_score = 0;

    for y in 0..ymax {
        for x in 0..xmax {
            let scores = get_vision_scores(&vision_map, &(x, y), xmax, ymax);

            let score = scores.top * scores.right * scores.bottom * scores.left;
            max_score = max(score, max_score);
        }
    }

    String::from(max_score.to_string())
}

fn get_vision_scores(vision_map: &VisionMap, point: &Point, xmax: i32, ymax: i32) -> VisionScores {
    let (x, y) = *point;
    let top = y as usize;
    let left = x as usize;
    let right = (xmax - 1 - x) as usize;
    let bottom = (ymax - 1 - y) as usize;

    VisionScores {
        top: vision_map.top[left][bottom],
        bottom: vision_map.bottom[left][top],
        left: vision_map.left[top][right],
        right: vision_map.right[top][left],
    }
}

fn get_grid_limits(grid: &Vec<Vec<u32>>) -> (i32, i32) {
    let xmax = grid.get(0).map_or(0, |xs| xs.len()) as i32;
    let ymax = grid.len() as i32;

    (xmax, ymax)
}

fn get_vision_map(grid: &Vec<Vec<u32>>, inclusive: bool) -> VisionMap {
    VisionMap {
        top: calculate_dir_vision_map(grid, Direction::Top, inclusive),
        bottom: calculate_dir_vision_map(grid, Direction::Bottom, inclusive),
        left: calculate_dir_vision_map(grid, Direction::Left, inclusive),
        right: calculate_dir_vision_map(grid, Direction::Right, inclusive),
    }
}

fn calculate_dir_vision_map(grid: &Vec<Vec<u32>>, dir: Direction, inclusive: bool) -> DirVisionMap {
    let (xmax, ymax) = get_grid_limits(grid);
    let offset_max = match dir {
        Direction::Top | Direction::Bottom => xmax,
        Direction::Left | Direction::Right => ymax,
    };

    let vision_len = match dir {
        Direction::Top | Direction::Bottom => ymax as usize,
        Direction::Left | Direction::Right => xmax as usize,
    };
    
    let mut dir_vision_map: Vec<Vec<u32>> = Vec::new();


    for offset in 0..offset_max {
        let DirTraversalParams {
            point: (x, y),
            dx,
            dy,
        } = get_dir_traversal_params(dir, xmax, ymax, offset);

        let mut cx = x;
        let mut cy = y;
        let mut i = 0;
        let mut stack: Vec<(u32, usize)> = vec![]; // Monotonic stack (num, index)
        let mut vision: Vec<u32> = vec![0; vision_len];

        while cx >= 0 && cy >= 0 && cx < xmax && cy < ymax {
            let num = grid[cy as usize][cx as usize];

            while ! stack.is_empty() && stack.last().unwrap().0 <= num {
                let (_, idx) = stack.pop().unwrap();
                if inclusive {
                    vision[idx] = i - (idx as u32);
                } else {
                    vision[idx] = i - (idx as u32) - 1;
                }
            }

            stack.push((num, i as usize));

            cx += dx;
            cy += dy;
            i += 1;
        }

        while ! stack.is_empty() {
            let (_, idx) = stack.pop().unwrap();
            vision[idx] = (vision_len - 1 - idx) as u32;
        }

        dir_vision_map.push(vision);
    }

    dir_vision_map
}

struct DirTraversalParams {
    point: Point,
    dx: i32,
    dy: i32,
}

fn get_dir_traversal_params(dir: Direction, xmax: i32, ymax: i32, offset: i32) -> DirTraversalParams {
    match dir {
        Direction::Top => DirTraversalParams { point: (offset, ymax - 1), dx: 0, dy: -1 },
        Direction::Right => DirTraversalParams { point: (0, offset), dx: 1, dy: 0 },
        Direction::Bottom => DirTraversalParams { point: (offset, 0), dx: 0, dy: 1 },
        Direction::Left => DirTraversalParams { point: (xmax - 1, offset), dx: -1, dy: 0 },
    }
}
