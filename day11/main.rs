use std::{fs, str::FromStr};

type Item = i64;

#[derive(Debug, Clone)]
enum MonkeyOpVal {
    Old,
    Val(i64),
}

#[derive(Debug, Clone)]
enum MonkeyOp {
    Mul(MonkeyOpVal, MonkeyOpVal),
    Add(MonkeyOpVal, MonkeyOpVal),
}

#[derive(Debug)]
struct MonkeyOpParseError;

impl FromStr for MonkeyOp {
    type Err = MonkeyOpParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.trim().split(" ").collect();

        let parse_op_val = |x: &str| match x {
            "old" => MonkeyOpVal::Old,
            s     => MonkeyOpVal::Val(s.parse().unwrap()),
        };

        let result = match parts[1] {
            "+" => MonkeyOp::Add(parse_op_val(parts[0]), parse_op_val(parts[2])),
            "*" => MonkeyOp::Mul(parse_op_val(parts[0]), parse_op_val(parts[2])),
            _   => return Err(MonkeyOpParseError),
        };

        Ok(result)
    }
}

impl MonkeyOp {
    fn execute(&self, old: i64) -> i64 {

        let get_val = |v: &MonkeyOpVal| match v {
            &MonkeyOpVal::Old    => old,
            &MonkeyOpVal::Val(n) => n as i64,
        };

        match self {
            MonkeyOp::Mul(a, b) => get_val(a) * get_val(b),
            MonkeyOp::Add(a, b) => get_val(a) + get_val(b),
        }
    }
}

#[derive(Debug, Clone)]
enum MonkeyTestOp {
    DivisibleBy(i64),
}

#[derive(Debug, Clone)]
struct MonkeyTestOpParseError;

impl FromStr for MonkeyTestOp {
    type Err = MonkeyTestOpParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.trim().starts_with("divisible by") {
            return Ok(
                MonkeyTestOp::DivisibleBy(
                    input.split(" ").last().unwrap().parse().unwrap()
                )
            );
        }

        Err(MonkeyTestOpParseError)
    }
}

#[derive(Debug, Clone)]
struct MonkeyTest {
    op: MonkeyTestOp,
    if_true: u32,
    if_false: u32,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: MonkeyOp,
    test: MonkeyTest,
}

#[derive(Debug)]
struct ThrowResult(Item, u32);

impl Monkey {
    fn inspect_item(&mut self, does_get_bored: bool) -> bool {
        let item = match self.items.last_mut() {
            None       => return false,
            Some(data) => data,
        };

        // Monkey inspects the first item
        *item = self.operation.execute(*item);

        if does_get_bored {
            // Monkey gets bored with the item
            *item = *item / 3;
        }

        true
    }

    fn throw(&mut self) -> Option<ThrowResult> {
        let item_to_throw = match self.items.pop() {
            None       => return None,
            Some(data) => data,
        };


        let target_monkey = self.test.run(item_to_throw);

        Some(
            ThrowResult(
                item_to_throw,
                target_monkey,
            )
        )
    }

    fn receive(&mut self, item: i64) -> () {
        self.items.push(item);
    }
}

impl MonkeyTest {
    fn execute(&self, val: i64) -> i64 {
        match self.op {
            MonkeyTestOp::DivisibleBy(n) => val % (n as i64),
        }
    }

    fn run(&self, val: i64) -> u32 {

        if self.execute(val) == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}


fn parse(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines().rev().collect::<Vec<&str>>();
    let mut monkies: Vec<Monkey> = Vec::new();

    while lines.len() >= 6 {
        // Monkey index
        lines.pop();

        let line2 = lines.pop().unwrap().trim();
        let items: Vec<i64> = line2.split(":")
            .collect::<Vec<&str>>()[1]
            .split(",")
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let line3 = lines.pop().unwrap().trim();
        let parsed3 = line3.split(":")
            .collect::<Vec<&str>>()[1]
            .split("=")
            .collect::<Vec<&str>>()[1];

        let op: MonkeyOp = parsed3.parse().unwrap();

        let line4 = lines.pop().unwrap().trim();
        let test_op: MonkeyTestOp = line4.split(":")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let line5 = lines.pop().unwrap().trim();
        let if_true: u32 = line5.split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let line6 = lines.pop().unwrap().trim();
        let if_false: u32 = line6.split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        monkies.push(Monkey {
            items,
            operation: op,
            test: MonkeyTest {
                op: test_op,
                if_true,
                if_false,
            }
        });


        while lines.len() != 0 {
            let last = lines.last().unwrap().trim();

            if last.len() == 0 {
                lines.pop();
            } else {
                break;
            }
        }
    }

    monkies
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed = parse(&contents);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}


fn part1(monkies_orig: &Vec<Monkey>) -> String {
    let mut monkies = monkies_orig.clone();
    let mut inspected_count = vec![0; monkies.len()];

    for _ in 0..20 {
        for i in 0..monkies.len() {
            loop {
                let inspected = monkies[i].inspect_item(true);

                if ! inspected {
                    break;
                }

                inspected_count[i] += 1;
                if let Some(ThrowResult(item, target_monkey_idx)) = monkies[i].throw() {
                    monkies[target_monkey_idx as usize].receive(item);
                }
            }
        }
    }

    inspected_count.sort();
    inspected_count.reverse();
    let result = inspected_count[0] * inspected_count[1];

    String::from(result.to_string())
}

fn part2(monkies_orig: &Vec<Monkey>) -> String {
    let mut monkies = monkies_orig.clone();
    let mut inspected_count: Vec<i64> = vec![0; monkies.len()];

    let monkies_primes_divisor = monkies.iter().map(|mok| {
        match mok.test.op {
            MonkeyTestOp::DivisibleBy(n) => n,
        }
    }).fold(1, |a, b| a * b);

    for _ in 0..10000 {
        for i in 0..monkies.len() {
            loop {
                let inspected = monkies[i].inspect_item(false);

                if ! inspected {
                    break;
                }

                let last = monkies[i].items.last_mut().unwrap();
                *last = *last % monkies_primes_divisor;

                inspected_count[i] += 1;
                if let Some(ThrowResult(item, target_monkey_idx)) = monkies[i].throw() {
                    monkies[target_monkey_idx as usize].receive(item);
                }
            }
        }
    }

    inspected_count.sort();
    inspected_count.reverse();
    let result = inspected_count[0] * inspected_count[1];

    String::from(result.to_string())
}
