use std::{fs, str::FromStr, collections::HashSet, iter::FromIterator};

#[derive(Debug, Clone, Copy)]
struct Range {
    s: i32,
    e: i32,
}

impl Range {
    fn merge(list: &mut Vec<Range>) -> Vec<Range> {
        let l = list.len();
        list.sort_by_key(|x| x.s);

        if l < 2 {
            return list.clone();
        }

        let mut result: Vec<Range> = vec![];

        let mut acc = list[0];
        for i in 1..l {
            let curr = list[i];

            if acc.e >= (curr.s - 1) {
                acc.e = i32::max(acc.e, curr.e); // Extend acc
            } else {
                result.push(acc);
                acc = curr;
            }
        }

        result.push(acc);

        result
    }
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance_to(&self, point: &Point) -> i32 {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_raw, y_raw) = s
            .split_once(",")
            .ok_or(ParsePointError)?;

        let parse = |s: &str, pfx: &str| 
            s.trim()
            .strip_prefix(pfx)
            .and_then(|y| y.parse::<i32>().ok())
            .ok_or(ParsePointError);

        let x = parse(x_raw, "x=")?;
        let y = parse(y_raw, "y=")?;
            
        Ok(Point { x, y })
    }
}

#[derive(Debug)]
struct SensorReport {
    sensor: Point,
    beacon: Point,
}

impl SensorReport {
    fn get_x_covered_range_for_y(&self, y: i32) -> Option<Range> {
        let Self { sensor, beacon } = self;
        let dist = sensor.manhattan_distance_to(beacon);
        let y_dist = (sensor.y - y).abs();

        let x_dist = dist - y_dist;

        if y_dist > dist { // Out of range
            return None;
        }

        Some(Range { s: sensor.x - x_dist, e: sensor.x + x_dist })
    }
}

fn parse(input: &str) -> Vec<SensorReport> {
    input.trim()
        .lines()
        .map(|l| {
            let (sensor_raw, beacon_raw) = l.split_once(":").expect("Invalid line format");

            let sensor = sensor_raw
                .trim()
                .strip_prefix("Sensor at")
                .and_then(|s| s.parse::<Point>().ok())
                .expect("Parsing sensor coords failed");

            let beacon = beacon_raw
                .trim()
                .strip_prefix("closest beacon is at")
                .and_then(|s| s.parse::<Point>().ok())
                .expect("Parsing beacon coords failed");

            SensorReport { sensor, beacon }
        })
        .collect()
}

const INPUT_FILE_NAME: &str = "./input";

fn main() {
    let contents = fs::read_to_string(INPUT_FILE_NAME)
        .expect("File not found");

    let parsed = parse(&contents);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn part1(reports: &Vec<SensorReport>) -> String {
    // The row to check
    // NOTE: Make sure you change this when switching between the ./input and ./exampleinput files
    let row = if INPUT_FILE_NAME.contains("example") { 10 } else { 2000000 };
    let ranges: Vec<Range> = Range::merge(
        &mut reports
            .iter()
            .filter_map(|sr| sr.get_x_covered_range_for_y(row))
            .collect()
    );

    let beacons_present = HashSet::<Point>::from_iter(reports.iter().map(|x| x.beacon))
        .iter()
        .filter(|Point { x, y }| {
            if *y != row {
                return false;
            }

            ranges.iter().any(|&Range { s, e }| s <= *x && *x <= e)
        }).count();

    let positions_count: i32 = ranges
        .iter()
        .map(|Range { s, e }| e - s + 1)
        .sum();
    
    let result = positions_count - (beacons_present as i32);

    String::from(result.to_string())
}

fn part2(reports: &Vec<SensorReport>) -> String {
    let mut holes = HashSet::<Point>::new();
    let max = if INPUT_FILE_NAME.contains("example") { 20 } else { 4000000 };

    for y in 0..max {
        let ranges: Vec<Range> = Range::merge(
            &mut reports
                .iter()
                .filter_map(|sr| sr.get_x_covered_range_for_y(y))
                .collect()
        );

        for i in 0..(ranges.len().checked_sub(1).unwrap_or(0)) {
            let (curr, next) = (ranges[i], ranges[i + 1]);
            
            for x in (curr.e + 1)..next.s {
                holes.insert(Point { x, y });
            }
        }
    }

    let distress_beacons: Vec<Point> = holes.into_iter().collect();

    if distress_beacons.len() > 1 {
        panic!("Found multiple distress beacons!");
    }

    let beacon = distress_beacons.get(0).expect("Distress beacon was not found1");

    let result = beacon.x as u64 * 4000000 + beacon.y as u64;

    String::from(result.to_string())
}
