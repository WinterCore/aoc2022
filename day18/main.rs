use std::{fs, collections::HashSet, iter::FromIterator};

#[derive(Debug, Hash, PartialEq, Eq)]
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
    println!("Part 2: {}", part2());
}


fn part1(droplets: &Vec<Point3D>) -> String {
    let map: HashSet<_> = HashSet::from_iter(droplets);

    let mut surface_area = 0u32;

    for droplet in droplets {
        let faces = droplet
            .get_neighbors()
            .iter()
            .map(|x| map.contains(x))
            .filter(|x| ! x)
            .count();

        surface_area += faces as u32;
    }

    String::from(surface_area.to_string())
}


fn part2() -> String {
    String::from("")
}
