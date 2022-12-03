use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum WinState {
    Left, // I lose
    Right, // I win
    Draw,
}

type Round = (Shape, Shape);

fn parse_shape(input: &str) -> Option<Shape> {
    match input {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissors),
        _ => None
    }
}

fn parse_winstate(input: &str) -> Option<WinState> {
    match input {
        "X" => Some(WinState::Left),
        "Y" => Some(WinState::Draw),
        "Z" => Some(WinState::Right),
        _ => None
    }
}

fn get_shape_points(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_state_shape(enemy: &Shape, desired_state: &WinState) -> Shape {
    let options = vec![
        Shape::Rock,
        Shape::Paper,
        Shape::Scissors,
    ];

    let all_states: Vec<(Shape, WinState)> = options
        .into_iter()
        .map(|x| (x, get_winner(&(enemy.clone(), x.clone()))))
        .collect();

    all_states.into_iter().find(|(_, w)| w == desired_state).unwrap().0
}

fn get_winner(round: &Round) -> WinState {
    let left_wins = |left, right| {
        match left {
            Shape::Rock => right == Shape::Scissors,
            Shape::Paper => right == Shape::Rock,
            Shape::Scissors => right == Shape::Paper,
        }
    };

    if left_wins(round.0, round.1) {
        return WinState::Left;
    }

    if left_wins(round.1, round.0) {
        return WinState::Right;
    }

    WinState::Draw
}

fn calculate_points(round: &Round) -> (i32, i32) {
    let winner = get_winner(&round);

    let left_points = get_shape_points(&round.0);
    let right_points = get_shape_points(&round.1);

    match winner {
        WinState::Draw => (left_points + 3, right_points + 3),
        WinState::Left => (left_points + 6, right_points),
        WinState::Right => (left_points, right_points + 6),
    }
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed: Vec<(String, String)> = contents
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|xs| xs
             .split(" ")
             .collect::<Vec<&str>>()
         )
        .filter(|xs| xs.len() == 2)
        .map(|x| (x[0].to_string(), x[1].to_string()))
        .collect();

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}


fn part1(input: &Vec<(String, String)>) -> String {
    let result: i32 = input
        .into_iter()
        .map(|t| (parse_shape(&t.0).unwrap(), parse_shape(&t.1).unwrap()))
        .map(|x| calculate_points(&x).1)
        .sum();

    result.to_string()
}

fn part2(input: &Vec<(String, String)>) -> String {
    let result: i32 = input
        .into_iter()
        .map(|t| (parse_shape(&t.0).unwrap(), parse_winstate(&t.1).unwrap()))
        .map(|(s, d)| calculate_points(&(s, get_state_shape(&s, &d))).1)
        .sum();

    result.to_string()
}
