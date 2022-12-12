use std::fs;
use std::mem;

/**
 * This shit was a disaster. Implementing a 2 way tree in rust seems to be impossible,
 * and it took me like 5 hours to implement a basic zipper as an alternative.
 */

#[derive(Debug)]
enum Node {
    File(String, usize),
    Dir(String, Vec<Node>)
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
    Dir(String),
    File(String, usize),
}

#[derive(Debug)]
struct FileZipper {
    node: Node,
    crumbs: Vec<Node>,
}

impl FileZipper {
    fn add_node(&mut self, node: Node) {
        if let Node::Dir(_, children) = &mut self.node {
            children.push(node);
        }
    }

    fn change_directory(&mut self, name: &str) {
        if name == ".." {
            let parent = self.crumbs.pop();
            if let Some(node) = parent {
                let curr = mem::replace(&mut self.node, node);

                if let Node::Dir(_, children) = &mut self.node {
                    children.push(curr);
                }
            }
        } else {
            if let Node::Dir(curr_name, children) = &mut self.node {
                let idx = children.iter().position(|x| {
                    match x {
                        Node::Dir(n, _) => n.eq(name),
                        _ => false,
                    }
                });

                if let None = idx {
                    return;
                }

                let item = children.swap_remove(idx.unwrap());
                let mut new_children = Vec::<Node>::new();
                while children.len() > 0 {
                    new_children.push(children.pop().unwrap());
                }
                self.crumbs.push(Node::Dir(curr_name.clone(), new_children));
                self.node = item;
            }
        }
    }
}

fn parse(cmds_str: Vec<&str>) -> Vec<Command> {

    let parse_command = |x: &&str| {
        if x.starts_with("$") {
            if x[2..].starts_with("cd") {
                let dir = x.split_whitespace().collect::<Vec<&str>>();
                return Some(
                    Command::Cd(
                        String::from(*dir.last().unwrap())
                    )
                );
            }

            if x[2..].starts_with("ls") {
                return Some(Command::Ls);
            }

            return None;
        }

        if x.starts_with("dir") {
            return Some(Command::Dir(String::from(&x[4..])))
        }

        if x.chars().nth(0).unwrap().is_digit(10) {
            let split = x.split_whitespace().collect::<Vec<&str>>();

            return Some(
                Command::File(
                    String::from(split[1]),
                    split[0].parse().unwrap(),
                )
            );
        }

        None
    };

    cmds_str.iter().filter_map(parse_command).collect()
}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("File not found");

    let lines = contents
        .trim()
        .lines()
        .collect::<Vec<&str>>();

    let commands = parse(lines);

    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands));
}

fn get_folder_sizes(node: &Node, sizes: &mut Vec<usize>) -> usize {
    match node {
        Node::File(_, size) => *size,
        Node::Dir(_, children) => {
            let size = children.iter().map(|x| get_folder_sizes(x, sizes)).sum();
            sizes.push(size);
            size
        },
    }
}

fn create_filesystem(insts: &Vec<Command>) -> FileZipper {
    let root = Node::Dir(String::from("/"), Vec::new());

    let mut zipper = FileZipper {
        node: root,
        crumbs: Vec::new(),
    };

    for inst in insts[1..].iter() {
        match inst {
            Command::Cd(dir) => zipper.change_directory(dir),
            Command::Dir(name) => zipper.add_node(Node::Dir(String::from(name), Vec::new())),
            Command::File(name, size) => zipper.add_node(Node::File(String::from(name), *size)),
            _ => (),
        }
    }

    while zipper.crumbs.len() > 0 {
        zipper.change_directory("..");
    }

    zipper
}

fn part1(insts: &Vec<Command>) -> String {
    let zipper = create_filesystem(insts);

    let mut sizes = Vec::new();

    get_folder_sizes(&zipper.node, &mut sizes);

    let result: usize = sizes.iter().filter(|&&x| x < 100000).sum();

    String::from(result.to_string())
}

fn part2(insts: &Vec<Command>) -> String {

    let zipper = create_filesystem(insts);

    let mut sizes = Vec::new();

    get_folder_sizes(&zipper.node, &mut sizes);
    sizes.sort();

    let total = 70000000;
    let used = sizes.last().unwrap();
    let free = total - used;
    let required = 30000000;

    if free > required {
        return String::from(0.to_string());
    }

    let needed = required - free;
    let folder_to_be_deleted = sizes.iter().find(|&&x| x > needed);

    String::from(folder_to_be_deleted.unwrap().to_string())
}
