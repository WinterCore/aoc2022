use core::time;
use std::{fs, thread::sleep};
use ncurses::*;

#[derive(Debug)]
enum Inst {
    Noop,
    Add(i32),
}

impl Inst {
    fn cycle_count(&self) -> u32 {
        match self {
            Inst::Noop   => 1,
            Inst::Add(_) => 2,
        }
    }
}

fn parse(input: &str) -> Vec<Inst> {
    input.trim()
        .lines()
        .map(|l| {
             let parts: Vec<&str> = l.trim().split(" ").collect();

             match parts[0] {
                 "addx" => Inst::Add(parts[1].parse().unwrap()),
                 "noop" => Inst::Noop,
                 _      => panic!("Invalid instruction"),
             }
        }).collect()
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let parsed = parse(&contents);

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed, true));
}

type RegisterState = (
    i32,   // Register Value
    usize, // Cycle number 
);

fn part1(insts: &Vec<Inst>) -> String {
    let mut cycle: usize = 1;
    let mut register_states: Vec<RegisterState> = vec![(1, cycle)];

    for inst in insts {
        cycle += inst.cycle_count() as usize;

        let (prev_register_val, _) = register_states.last().unwrap();

        match inst {
            Inst::Noop   => register_states.push((*prev_register_val, cycle)),
            Inst::Add(v) => register_states.push((prev_register_val + v, cycle)),
        }
    }

    let result: i32 = (0..6)
        .step_by(1)
        .map(|i| {
            let cycle = i * 40 + 20;
            get_cycle_register_state(&register_states, cycle) * cycle as i32
        }).sum();

    String::from(result.to_string())
}


const SPRITE_SIZE: i32 = 3;
const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;
const CRT_SIZE: usize = CRT_WIDTH * CRT_HEIGHT;

fn part2(insts: &Vec<Inst>, render: bool) -> String {
    let mut cycle: usize = 1;
    let mut sprite_pos: i32 = 1; // The value of the X CPU register (the sprite position)

    if render {
        initscr();
    }

    let mut screen_buffer = vec!['.'; CRT_SIZE];
    let win = newwin(CRT_HEIGHT as i32, CRT_WIDTH as i32, 0, 0);


    for inst in insts {
        let inst_cycle_count = inst.cycle_count() as usize;

        for _ in 1..=inst_cycle_count {
            let cycle_pos = (cycle % CRT_WIDTH) as i32;

            if (cycle_pos >= sprite_pos && cycle_pos < (sprite_pos + SPRITE_SIZE))
               && (cycle_pos != 0) {
                screen_buffer[cycle - 1] = '@';
            }

            if render {
                wclear(win);
                screen_buffer_to_str(&screen_buffer)
                    .iter()
                    .enumerate()
                    .for_each(|(i, line)| {
                        mvwaddstr(win, i as i32, 0, line);
                    });



                mvwaddch(win, (cycle / CRT_WIDTH) as i32, cycle_pos, '|' as u32);
                wrefresh(win);

                sleep(time::Duration::from_millis(100));
            }

            cycle += 1;
        }

        sprite_pos = match inst {
            Inst::Add(v) => sprite_pos + v,
            _            => sprite_pos,
        }
    }

    if render {
        endwin();
    }

    let mut output = String::from("\n");
    output.push_str(&screen_buffer_to_str(&screen_buffer).join("\n"));
    output
}

fn screen_buffer_to_str(screen_buffer: &Vec<char>) -> Vec<String> {
    screen_buffer
        .chunks(CRT_WIDTH)
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<String>>()
}


// Scuffed binary search to find what the value of the register was at a specific cycle
fn get_cycle_register_state(register_states: &Vec<RegisterState>, cycle: usize) -> i32 {
    let mut s = 0;
    let mut e = register_states.len();

    while s < e {
        let mid = (e - s) / 2 + s;
        let (val, cc) = register_states[mid];

        if cc == cycle {
            return val;
        }

        if cc < cycle {
            s = mid + 1;            
            continue;
        }

        if cc > cycle {
            e = mid;
            continue;
        }
    }

    register_states[e - 1].0
}
