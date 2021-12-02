use std::fs::read_to_string;

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse_line(line: &str) -> Option<Command> {
    let command = line.split(' ').next()?;
    let value = line
        .split(' ')
        .skip(1)
        .next()
        .and_then(|v| v.parse::<usize>().ok())?;
    match command {
        "forward" => Some(Command::Forward(value)),
        "down" => Some(Command::Down(value)),
        "up" => Some(Command::Up(value)),
        _ => None,
    }
}

fn part_1(commands: &[Command]) -> usize {
    let mut position = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Command::Forward(v) => position += v,
            Command::Down(v) => depth += v,
            Command::Up(v) => depth -= v,
        }
    }
    position * depth
}

fn part_2(commands: &[Command]) -> usize {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Forward(v) => {
                position += v;
                depth += aim * v;
            }
            Command::Down(v) => aim += v,
            Command::Up(v) => aim -= v,
        }
    }
    position * depth
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let commands = input
        .lines()
        .map(parse_line)
        .map(|cmd| cmd.expect("Cannot parse command"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&commands));
    println!("Part 2: {}", part_2(&commands));
}
