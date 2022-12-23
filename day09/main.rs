use std::{fs, collections::HashSet, thread::sleep, time};

type Point = (i32, i32); // x, y
type MoveDelta = (i32, i32); // dx, dy

#[derive(Debug)]
enum MoveInstruction {
    Up(u32),
    Right(u32),
    Down(u32),
    Left(u32),
}


fn parse(input: &str) -> Vec<MoveInstruction> {
    let to_move_inst = |d: &str, l: &str| -> Option<MoveInstruction> {
        let distance = l.parse().unwrap();

        match d {
            "U" => Some(MoveInstruction::Up(distance)),
            "R" => Some(MoveInstruction::Right(distance)),
            "D" => Some(MoveInstruction::Down(distance)),
            "L" => Some(MoveInstruction::Left(distance)),
            _ => None
        }
    };

    let data = input
        .trim()
        .lines()
        .map(|x| x.trim().split(" ").collect::<Vec<&str>>())
        .filter_map(|x| to_move_inst(x.get(0).unwrap(), x.get(1).unwrap()))
        .collect();

    data
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed = parse(&contents);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn delta_to_single_move(delta: &MoveDelta) -> MoveDelta {
    let (x, y) = *delta;

    let to_single = |v: i32| { 
        if v == 0 {
            return 0;
        }

        if v.is_negative() {
            return -1;
        }

        1
    };

    (to_single(x), to_single(y))
}

fn part1(move_insts: &Vec<MoveInstruction>) -> String {
    let mut visited_set = HashSet::<Point>::new();

    let mut head: Point = (0, 0);
    let mut tail: Point = (0, 0);

    for inst in move_insts {
        let (mut dx, mut dy) = move_inst_to_move_delta(&inst);

        // println!("Executing {:?} | {:?} {:?}\n", inst, dx, dy);
        // Execute one move at a time
        while dx != 0 || dy != 0 {
            let (sdx, sdy) = delta_to_single_move(&(dx, dy));
            head = move_point(&head, &(sdx, sdy));
                
            let FollowResult {
                position,
                points_visited,
            } = follow_point(&tail, &head, 1);

            tail = position;
            points_visited
                .iter()
                .for_each(|p| {
                    visited_set.insert(*p);
                });

            dx -= sdx;
            dy -= sdy;
            // println!("dx: {:?}, dy: {:?}", dx, dy);
        }
    }

    // println!("{:?}", visited_set);

    String::from((visited_set.len() + 1).to_string()) // Add one because of the starting point
}

fn part2(move_insts: &Vec<MoveInstruction>) -> String {
    String::from("")
}

fn move_inst_to_move_delta(move_inst: &MoveInstruction) -> MoveDelta {
    match move_inst {
        MoveInstruction::Up(d) => (0, *d as i32),
        MoveInstruction::Right(d) => (*d as i32, 0),
        MoveInstruction::Down(d) => (0, (*d as i32) * -1),
        MoveInstruction::Left(d) => ((*d as i32) * -1, 0),
    }
}

fn move_point(point: &Point, move_delta: &MoveDelta) -> Point {
    let (x, y) = *point;
    let (dx, dy) = *move_delta;

    (x + dx, y + dy)
}

struct FollowResult {
    position: Point,
    points_visited: Vec<Point>,
}

fn follow_point(point: &Point, target: &Point, max_allowed_distance: u32) -> FollowResult {
    let mut points_visited: Vec<Point> = vec![];
    let mut position: Point = *point;

    // println!("New call");

    loop {
        let mut distance = get_dir_distance(&target, &position);
        let Distance { distance, dx, dy } = distance;
        // println!("{:?} {:?} {:?}", position, target, distance);
        // sleep(time::Duration::from_millis(20));

        if distance <= max_allowed_distance {
            break;
        }

        if dx != 0 && dy != 0 {
            let delta = delta_to_single_move(&(dx, dy));
            position = move_point(&position, &delta);
            points_visited.push(position);

            continue;
        }

        
        if dx != 0 {
            position = move_point(&position, &delta_to_single_move(&(dx, 0)));
            points_visited.push(position);

            continue;
        }

        if dy != 0 {
            position = move_point(&position, &delta_to_single_move(&(0, dy)));
            points_visited.push(position);

            continue;
        }
    }

    FollowResult {
        position,
        points_visited,
    }
}

struct Distance {
    distance: u32,
    dx: i32,
    dy: i32,
}

fn get_dir_distance(a: &Point, b: &Point) -> Distance {
    let (x1, y1) = *a;
    let (x2, y2) = *b;

    let (dx, dy) = (x1 - x2, y1 - y2);
    let distance = (((dx).pow(2) + (dy).pow(2)) as f32).sqrt();

    Distance {
        dx,
        dy,
        distance: distance.floor() as u32,
    }
}
