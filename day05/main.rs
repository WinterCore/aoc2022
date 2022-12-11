use std::fs;

#[derive(Debug)]
struct Instruction {
    amount: i32,
    source: i32,
    destination: i32,
}

type Stack<T> = Vec<T>;

fn parse_stacks(input: &str) -> Vec<Stack<char>> {
    let raw_stacks = input.split("\n").collect::<Vec<&str>>();
    let (rows, identifiers) = raw_stacks.split_at(raw_stacks.len() - 1);
    let stacks_count = identifiers[0].trim().split_whitespace().count();

    let chars = rows
        .into_iter()
        .map(|x| String::from(*x))
        .map(|row| row.chars().collect::<Vec<char>>())
        .rev()
        .collect::<Vec<Vec<char>>>();

    let rows = chars
        .iter()
        .map(|xs| xs.chunks(4).map(|x| x[1]).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut stacks = vec![0; stacks_count].iter().map(|_| Vec::new()).collect::<Vec<Vec<char>>>();

    for row in rows.iter() {
        for i in 0..stacks_count {
            let c = row[i];
            
            if c.is_whitespace() {
                continue;
            }

            stacks[i].push(c);
        }
    }

    stacks
}

fn parse_insts(input: &str) -> Vec<Instruction> {
    let lines = input.split("\n");

    lines.filter_map(|x| {
        let split = x.split_whitespace().collect::<Vec<&str>>();

        if split.len() != 6 {
            return None;
        }

        Some(Instruction {
            amount: split[1].parse().unwrap(),
            source: split[3].parse().unwrap(),
            destination: split[5].parse().unwrap(),
        })
    }).collect()
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let data = contents.split("\n\n").collect::<Vec<&str>>();
    let stacks = parse_stacks(data[0]);
    let instructions = parse_insts(data[1]);

    println!("Part 1: {}", part1(&stacks, &instructions));
    println!("Part 2: {}", part2(&stacks, &instructions));
}


fn part1(stacks: &Vec<Stack<char>>, insts: &Vec<Instruction>) -> String {
    let mut cloned_stacks = stacks.clone();

    for inst in insts.iter() {
        for _ in 0..inst.amount {
            let popped = cloned_stacks[(inst.source - 1) as usize].pop();

            if let None = popped {
                break;
            }

            cloned_stacks[(inst.destination - 1) as usize].push(popped.unwrap());
        }
    }

    cloned_stacks.iter().map(|x| x.last().unwrap()).collect::<String>()
}

fn part2(stacks: &Vec<Stack<char>>, insts: &Vec<Instruction>) -> String {
    let mut cloned_stacks = stacks.clone();

    for inst in insts.iter() {
        let mut temp = Vec::new();
        for _ in 0..inst.amount {
            let popped = cloned_stacks[(inst.source - 1) as usize].pop();
            
            if let None = popped {
                break;
            }

            temp.push(popped.unwrap());
        }

        cloned_stacks[(inst.destination - 1) as usize].extend(temp.iter().rev());
    }

    cloned_stacks.iter().map(|x| x.last().unwrap()).collect::<String>()
}
