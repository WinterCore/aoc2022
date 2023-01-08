use core::time;
use std::{fs, str::FromStr, collections::{HashMap, HashSet}, thread::sleep, usize};


#[derive(Debug, Clone)]
struct Valve {
    name: String,
    rate: i32,
    connections: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseValveError;

impl FromStr for Valve {
    type Err = ParseValveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .trim()
            .split_once(";")
            .ok_or(ParseValveError)?;

        let (raw_name, raw_rate) = a
            .trim()
            .split_once(" has flow ")
            .ok_or(ParseValveError)?;

        let name = raw_name
            .trim()
            .strip_prefix("Valve ")
            .and_then(|s| Some(String::from(s)))
            .ok_or(ParseValveError)?;

        let rate = raw_rate
            .trim()
            .strip_prefix("rate=")
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or(ParseValveError)?;

        let connections = Some(b.trim())
            .and_then(|s| s.strip_prefix("tunnel ").or(s.strip_prefix("tunnels ")))
            .and_then(|s| s.strip_prefix("lead ").or(s.strip_prefix("leads ")))
            .and_then(|x| x.strip_prefix("to "))
            .and_then(|s| s.strip_prefix("valve ").or(s.strip_prefix("valves ")))
            .and_then(|s| Some(
                s.split(",")
                .map(|c| c.trim().to_string())
                .collect::<Vec<String>>()
            )).ok_or(ParseValveError)?;

        Ok(Valve { name, rate, connections })
    }
}

#[derive(Debug)]
struct SimpleValve {
    rate: i32,
    links: Vec<usize>,
}

fn parse(input: &str) -> Vec<SimpleValve> {
    let valves: Vec<Valve> = input.lines()
        .filter(|l| ! l.trim().is_empty())
        .map(|l| l.parse::<Valve>().unwrap())
        .collect();
    
    let idx_map: HashMap<String, usize> = valves.iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (i, x)| {
            m.insert(x.name.clone(), i);
            m
        });


    valves.into_iter()
        .map(|v| SimpleValve {
            rate: v.rate,
            links: v.connections.iter().map(|x| *idx_map.get(x).unwrap()).collect(),
        })
        .collect()
}

fn main() {
    let contents = fs::read_to_string("./exampleinput")
        .expect("File not found");

    let parsed = parse(&contents);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2());
}

fn part1(valves: &Vec<SimpleValve>) -> String {

    // println!("{:#?}", valves);
    let graph = init_graph(valves, |x| &x.links);

    let dists = floyd_warshall(graph);

    /*
    let hello = String::from("world");
    opened.insert(hello);
    */

    // let pressure = get_max_released_pressure(valve_map, &mut opened, "AA", 4);

    // println!("{:?}", pressure);

    String::from("")
}

fn part2() -> String {
    String::from("")
}

fn floyd_warshall(graph: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let l = graph.len();
    let mut dist = graph.clone();

    for k in 0..l {
      for i in 0..l {
        for j in 0..l {
          if dist[i][k] + dist[k][j] < dist[i][j] {
            dist[i][j] = dist[i][k] + dist[k][j];
          }
        }
      }
    }

    dist
}

fn init_graph<T, F>(list: &Vec<T>, get_links: F) -> Vec<Vec<usize>>
    where F: Fn(&T) -> &Vec<usize> {
    let l = list.len();
    let mut graph = vec![vec![usize::MAX / 2; l]; l];

    list
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            get_links(x).iter().for_each(|&j| graph[i][j] = 1);
        });

    graph
}

// Calculate profitability of all paths
// Loop over all paths and find the one that is most profitable and take into account the initial
// movement
