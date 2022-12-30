use core::time;
use std::thread::sleep;
use std::{fs};
use std::collections::{HashMap, VecDeque, HashSet};
use ncurses::*;

type Point = (i32, i32);

type PointInfo = (
    i32, // Distance
    Point, // Prev
);

struct ElevationMap {
    start: Point,
    end: Point,
    grid: Vec<Vec<char>>,
}

fn find_grid_point(grid: &Vec<Vec<char>>, needle: char) -> Option<Point> {
    for y in 0..grid.len() {
        let row = &grid[y];

        for x in 0..row.len() {
            if grid[y][x] == needle {
                return Some((x as i32, y as i32));
            }
        }
    }

    None
}

fn render_height_map(grid: &Vec<Vec<char>>, visited: &HashSet<Point>) {
    clear();

    let (xmax, ymax) = get_grid_limits(grid);

    for y in 0..ymax {
        for x in 0..xmax {
            let curr = grid[y as usize][x as usize];
            if visited.contains(&(x, y)) {
                addch('.' as u32);
            } else {
                addch(curr as u32);
            }
        }

        addch('\n' as u32);
    }


    refresh();
}

fn render_path(grid: &Vec<Vec<char>>, path: &Vec<Point>) {
    let mut buffer = grid.clone();

    let (xmax, ymax) = get_grid_limits(grid);

    for i in 0..(path.len() - 1) {
        let (x, y) = path[i]; // Current
        let (nx, ny) = path[i + 1]; // Next
        
        let cell = &mut buffer[y as usize][x as usize];

        *cell = '*';
    }

    clear();

    for y in 0..ymax {
        for x in 0..xmax {
            let curr = buffer[y as usize][x as usize];
            if curr == '*' {
                attron(COLOR_PAIR(1));
                addch(curr as u32);
                attroff(COLOR_PAIR(1));
            } else {
                addch(curr as u32);
            }
        }

        addch('\n' as u32);
    }

    addstr("\n\n");
    addstr("Press any key to proceed...");
    refresh();
    getch();
}

fn parse(input: &str) -> ElevationMap {
    let mut grid: Vec<Vec<char>> = input.trim()
        .lines()
        .map(|x| x.trim().chars().collect())
        .collect();

    let start = find_grid_point(&grid, 'S').expect("Starting point was not found");
    let end   = find_grid_point(&grid, 'E').expect("Destination point was not found");
    
    grid[start.1 as usize][start.0 as usize] = 'a';
    grid[end.1 as usize][end.0 as usize] = 'z';

    ElevationMap {
        start,
        end,
        grid,
    }
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed = parse(&contents);

    println!("Part 1: {}", part1(&parsed, true));
    println!("Part 2: {}", part2(&parsed));
}

fn part1(em: &ElevationMap, render: bool) -> String {
    let ElevationMap {
        start,
        end,
        grid,
    } = em;

    if render {
        initscr();
        start_color();
        init_pair(1, COLOR_RED, COLOR_BLACK);
    }

    // Dijkstra's algo
    let mut visited = HashSet::<Point>::new();
    let mut pim = HashMap::<Point, PointInfo>::new(); // Point info map
    let mut queue = VecDeque::<Point>::new();
    queue.push_back(*start);
    pim.insert(*start, (0, *start));
    visited.insert(*start);
    
    while queue.len() > 0 {
        let point = queue.pop_front().unwrap();

        let neighbors: Vec<Point> = get_point_neighbors(grid, &point)
            .into_iter()
            .filter(|p| ! visited.contains(p))
            .collect();
        let prev_point_info = *pim.get(&point).unwrap();

        if render {
            render_height_map(&grid, &visited);
        }

        for np in neighbors {
            let prev_distance = pim.get(&np).map_or(std::i32::MAX, |x| x.0);

            let curr_dist = prev_point_info.0 + 1;

            if curr_dist < prev_distance {
                pim.insert(np, (curr_dist, point));
            }

            queue.push_back(np);
            visited.insert(np);
        }
    }

    let shortest_path = get_shortest_path_from_pim(&pim, start, end);
    let len = shortest_path.len() - 1;

    if render {
        render_path(grid, &shortest_path);
        endwin();
    }

    String::from(len.to_string())
}

fn part2(grid: &ElevationMap) -> String {
    String::from("")
}

fn get_grid_limits<T>(grid: &Vec<Vec<T>>) -> (i32, i32) {
    let ymax = grid.len() as i32;
    let xmax = grid.get(0).map_or(0, |xs| xs.len()) as i32;

    (xmax, ymax)
}

fn get_point_neighbors(grid: &Vec<Vec<char>>, point: &Point) -> Vec<Point> {
    let (x, y) = *point;
    let (xmax, ymax) = get_grid_limits(grid);

    let neighbors = [
        (x, y - 1),
        (x + 1, y),
        (x, y + 1),
        (x - 1, y),
    ];

    neighbors
        .into_iter()
        .filter(|&(x, y)| x >= 0 && x < xmax && y >= 0 && y < ymax)
        .filter(|&(cx, cy)|
                grid[cy as usize][cx as usize] as u32
                <= grid[y as usize][x as usize] as u32 + 1
        ).collect()
}

fn get_shortest_path_from_pim(pim: &HashMap::<Point, PointInfo>, start: &Point, end: &Point) -> Vec<Point> {

    let mut current_point = end;
    let mut path = vec![*end];

    while *current_point != *start {
        let p = pim.get(current_point).unwrap();

        path.push(p.1);
        current_point = &p.1;
    }

    path.into_iter().rev().collect()
}
