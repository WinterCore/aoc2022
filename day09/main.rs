use std::{fs, collections::HashSet};
use ncurses::*;

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
    println!("Part 2: {}", part2(&parsed, false));

    // Enable the rope visualizer by uncomentting the following line
    /*
    initscr();
    part2(&parsed, true);
    endwin();
    */
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
    visited_set.insert((0, 0));

    let mut head: Point = (0, 0);
    let mut tail: Point = (0, 0);

    for inst in move_insts {
        let (mut dx, mut dy) = move_inst_to_move_delta(&inst);

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
        }
    }


    (visited_set.len() + 1).to_string()
}

fn part2(move_insts: &Vec<MoveInstruction>, render: bool) -> String {
    let mut visited_set = HashSet::<Point>::new();
    visited_set.insert((0, 0));

    let mut rope: Vec<Point> = vec![(0, 0); 10];

    for inst in move_insts {
        let (mut dx, mut dy) = move_inst_to_move_delta(&inst);

        // println!("Executing {:?} | {:?} {:?}\n", inst, dx, dy);
        // Execute one move at a time
        while dx != 0 || dy != 0 {
            let (sdx, sdy) = delta_to_single_move(&(dx, dy));
            // Move the head
            rope[0] = move_point(&rope[0], &(sdx, sdy));

            // Simulate the tail
            for i in 0..(rope.len() - 1) { 
                let (head, tail) = (rope[i], rope[i + 1]);
                    
                let FollowResult {
                    position,
                    points_visited,
                } = follow_point(&tail, &head, 1);

                rope[i + 1] = position;

                if i == rope.len() - 2 {
                    points_visited
                        .iter()
                        .for_each(|p| {
                            visited_set.insert(*p);
                        });
                }

            }

            if render {
                render_rope(&rope, &visited_set);
            }

            dx -= sdx;
            dy -= sdy;
        }
    }

    visited_set.len().to_string() // Add one because of the starting point
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

    loop {
        let distance = get_dir_distance(&target, &position);
        let Distance { distance, dx, dy } = distance;

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

fn render_rope(knots: &Vec<Point>, visited_set: &HashSet<Point>) {
    let xiter = knots.iter().map(|x| x.0);
    let yiter = knots.iter().map(|x| x.1);
    let (minx, miny) = (xiter.clone().min().unwrap(), yiter.clone().min().unwrap());
    let (maxx, maxy) = (xiter.clone().max().unwrap(), yiter.clone().max().unwrap());
    let buffery = 70;
    let bufferx = 150;
    let shiftx = minx * -1 + ((bufferx as i32) / 2) + (maxx) / 2;
    let shifty = miny * -1 + ((buffery as i32) / 2) + (maxy) / 2;
    let mut buffer: Vec<Vec<char>> = vec![vec!['.'; bufferx]; buffery];

    knots.iter()
        .enumerate()
        .rev()
        .for_each(|(i, (x, y))| {
            buffer[(shifty + y) as usize][(shiftx + x) as usize] = char::from_digit(i as u32, 10).unwrap();
        });

    visited_set.iter().for_each(|(x, y)| {
        buffer[(shifty + y) as usize][(shiftx + x) as usize] = '#';
    });

    let mut output = buffer
        .into_iter()
        .enumerate()
        .rev()
        .map(|(i, x)| {
            let n = (i as i32) - shifty;
            
            let mut output = String::new();
            if n == 0 {
                output.push_str(&n.to_string());
                output.push_str(" ");
            } else {
                output.push_str("  ");
            }

            output.push_str(&x.iter().collect::<String>());

            output
        }).collect::<Vec<String>>()
        .join("\n");

    output.push_str("\n  ");
    output.push_str(
        &(0..bufferx)
            .step_by(1)
            .map(|i| {
                let n = i as i32 - shiftx;
                if n == 0 {
                    return n.to_string();
                }

                String::from(" ")
            })
            .collect::<String>()
    );

    clear();

    addstr(&output);

    refresh();
    getch();
    // sleep(time::Duration::from_millis(10));
    
}
