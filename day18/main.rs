use std::{fs, collections::{HashSet, VecDeque}, iter::FromIterator};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn get_neighbors(&self) -> Vec<Point3D> {
        vec![
            // Z
            Point3D { x: self.x, y: self.y, z: self.z - 1 },
            Point3D { x: self.x, y: self.y, z: self.z + 1 },

            // X
            Point3D { x: self.x - 1, y: self.y, z: self.z },
            Point3D { x: self.x + 1, y: self.y, z: self.z },

            // Y
            Point3D { x: self.x, y: self.y - 1, z: self.z },
            Point3D { x: self.x, y: self.y + 1, z: self.z },
        ]
    }
}

fn parse(data: &str) -> Vec<Point3D> {
    data.trim()
        .lines()
        .map(|l| {
            let split: Vec<&str> = l.trim()
                .split(',')
                .collect();

            Point3D {
                x: split[0].parse().unwrap(),
                y: split[1].parse().unwrap(),
                z: split[2].parse().unwrap(),
            }
        }).collect()
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let droplets = parse(&contents);

    println!("Part 1: {}", part1(&droplets));
    println!("Part 2: {}", part2(&droplets));
}


fn part1(droplets: &Vec<Point3D>) -> String {
    let map: HashSet<_> = HashSet::from_iter(droplets);

    let mut surface_area = 0u32;

    for droplet in droplets {
        let faces = droplet
            .get_neighbors()
            .iter()
            .filter(|x| ! map.contains(x))
            .count();

        surface_area += faces as u32;
    }

    String::from(surface_area.to_string())
}

enum CoordType {
    Min,
    Max,
}

fn get_coord<'a, F>(
    map: &'a HashSet<&'a Point3D>,
    coord_type: CoordType,
    get_key: F,
) -> &'a Point3D where F: FnMut(&&&Point3D) -> i32 {
    let iter = map.into_iter();

    match coord_type {
        CoordType::Min => iter.min_by_key(get_key).unwrap(),
        CoordType::Max => iter.max_by_key(get_key).unwrap(),
    }
}

fn part2(droplets: &Vec<Point3D>, ) -> String {
    let map: HashSet<_> = HashSet::from_iter(droplets);

    let min_x = get_coord(&map, CoordType::Min, |p| p.x).x;
    let min_y = get_coord(&map, CoordType::Min, |p| p.y).y;
    let min_z = get_coord(&map, CoordType::Min, |p| p.z).z;

    let max_x = get_coord(&map, CoordType::Max, |p| p.x).x;
    let max_y = get_coord(&map, CoordType::Max, |p| p.y).y;
    let max_z = get_coord(&map, CoordType::Max, |p| p.z).z;

    let min = Point3D { x: min_x - 1, y: min_y - 1, z: min_z - 1 };
    let max = Point3D { x: max_x + 1, y: max_y + 1, z: max_z + 1 };

    let mut flood: HashSet<Point3D> = HashSet::new();
    let mut stack: VecDeque<Point3D> = VecDeque::new();

    flood.insert(min.clone());
    stack.push_back(min.clone());

    while let Some(point) = stack.pop_front() {
        let neighbors: Vec<Point3D> = point
            .get_neighbors()
            .into_iter()
            .filter(|p@Point3D { x, y, z }| {
                ! map.contains(p) &&
                ! flood.contains(p) &&
                min.x <= *x && *x <= max.x && 
                min.y <= *y && *y <= max.y &&
                min.z <= *z && *z <= max.z
            }).collect();


        neighbors.iter().for_each(|p| {
            stack.push_back(p.clone());
            flood.insert(p.clone());
        });
    }

    let mut surface_area = 0u32;

    for point in flood.iter() {
        let touched_faces = point
            .get_neighbors()
            .iter()
            .filter(|x| map.contains(x))
            .count();

        surface_area += touched_faces as u32;
    }

    String::from(surface_area.to_string())
}
