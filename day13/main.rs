use std::{fs, str::FromStr, cmp::{Ord, Ordering}};

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Value(i32),
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePacketError;

impl Packet {
    fn collect_list(s: &Vec<char>, i: usize) -> usize {

        let mut j = 0;
        let mut openings = 0;

        loop {
            let c = s[j + i];

            if c == '[' {
                openings += 1;
            }

            if c == ']' {
                openings -= 1;
            }

            if openings == 0 {
                break;
            }

            j += 1;
        }

        j
    }

    fn parse_num(s: &Vec<char>, i: &mut usize) -> Self {
        let digits = s[*i..].iter().take_while(|x| x.is_digit(10)).collect::<String>();

        let val = digits.parse().expect("Invalid number");

        *i += digits.len();

        Packet::Value(val)
    }

    fn consume_whitespace(s: &Vec<char>, i: &mut usize) -> () {
        if *i >= s.len() {
            return;
        }

        while s[*i].is_whitespace() { *i += 1 }
    }

    fn consume_comma(s: &Vec<char>, i: &mut usize) {
        if *i >= s.len() {
            return;
        }

        if s[*i] != ',' {
            panic!("Expected comma at pos {}", i);
        }

        *i += 1; // Consume the comma
    }

    fn to_list(&self) -> Vec<Self> {
        match self {
            Packet::Value(n) => vec![Packet::Value(*n)],
            Packet::List(l)  => l.clone(),
        }
    }

    fn parse_list(s: &Vec<char>, i: &mut usize) -> Self {
        let list_len = Packet::collect_list(s, *i);
        
        let mut list: Vec<Packet> = vec![];

        let j = *i;
        *i += 1; // Consume [
        while *i < j + list_len {
            Packet::consume_whitespace(s, i);

            if s[*i] == '[' {
                list.push(Packet::parse_list(s, i));
            } else if s[*i].is_digit(10) {
                list.push(Packet::parse_num(s, i));
            }

            Packet::consume_whitespace(s, i);

            if s[*i] == ']' {
                break;
            }

            Packet::consume_comma(s, i);
        }
        *i += 1; // Consume ]

        Packet::List(list)
    }

    fn parse(s: &str) -> Self {
        let mut i = 0;

        Packet::parse_list(&s.chars().collect(), &mut i)
    }
}

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        Ok(Packet::parse(trimmed))
    }
}
impl Eq for Packet {}

impl PartialOrd for Packet {
    // I have no clue why I this isn't implied automatically since I've implemented the Ord trait
    // Not having this makes the comparison operators (<, >, etc) not work properly
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => return a.cmp(b),
            _ => (),
        }
        

        let p1 = self.to_list();
        let p2 = other.to_list();

        let mut i = 0;
        let mut j = 0;

        while i < p1.len() && j < p2.len() {
            let ord = p1[i].cmp(&p2[i]);

            if ord != Ordering::Equal {
                return ord;
            }
            
            i += 1;
            j += 1;
        }

        if p1.len() < p2.len() {
            Ordering::Less
        } else if p2.len() < p1.len() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

type PacketPair = (Packet, Packet);

fn parse(input: &str) -> Vec<PacketPair> {
    input.trim()
        .split("\n\n")
        .map(|ppstr| {
             let pair = ppstr.split("\n").collect::<Vec<&str>>();
             (Packet::from_str(pair[0]).unwrap(), Packet::from_str(pair[1]).unwrap())
        })
        .collect()
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed = parse(&contents);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn part1(pps: &Vec<PacketPair>) -> String {
    let mut idxs: Vec<usize> = Vec::new();

    for (i, item) in pps.iter().enumerate() {
        let (p1, p2) = item;

        if p1 < p2 {
            idxs.push(i + 1);
        }
    }

    let result: usize = idxs.iter().sum();
    String::from(result.to_string())
}

fn part2(pps: &Vec<PacketPair>) -> String {
    let mut packets: Vec<Packet> = pps
        .iter()
        .flat_map(|x| vec![x.0.clone(), x.1.clone()])
        .collect();

    packets.sort_by(|x, y| x.cmp(y));

    let mut idxs: Vec<usize> = Vec::new();
    
    let divider_packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Value(6)])]),
    ];

    let mut i = 0; // packets index
    let mut j = 0; // divider_packets index
    
    while j < divider_packets.len() && i < packets.len() {
        if divider_packets[j] < packets[i] {
            idxs.push(i + j);
            j += 1;
        }

        i += 1;
    }

    let result: usize = idxs
        .iter()
        .map(|x| x + 1) // Add one cuz the challenge starts counting from 1
        .product();
    String::from(result.to_string())
}
