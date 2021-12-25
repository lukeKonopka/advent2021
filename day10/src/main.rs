use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq)]
enum Chunk {
    Paren,
    Bracket,
    Curly,
    Angle,
}

impl Chunk {
    fn get_invalid_score(&self) -> usize {
        match self {
            Chunk::Paren => 3,
            Chunk::Bracket => 57,
            Chunk::Curly => 1197,
            Chunk::Angle => 25137,
        }
    }

    fn get_autocomplete_score(&self) -> usize {
        match self {
            Chunk::Paren => 1,
            Chunk::Bracket => 2,
            Chunk::Curly => 3,
            Chunk::Angle => 4,
        }
    }
}

#[derive(Clone, Copy)]
enum Mark {
    Open(Chunk),
    Close(Chunk),
}

impl Mark {
    fn from_char(c: char) -> Mark {
        match c {
            '(' => Mark::Open(Chunk::Paren),
            '[' => Mark::Open(Chunk::Bracket),
            '{' => Mark::Open(Chunk::Curly),
            '<' => Mark::Open(Chunk::Angle),
            ')' => Mark::Close(Chunk::Paren),
            ']' => Mark::Close(Chunk::Bracket),
            '}' => Mark::Close(Chunk::Curly),
            '>' => Mark::Close(Chunk::Angle),
            _ => panic!("unknown chunk {}", c),
        }
    }
}

fn part_1(lines: &Vec<Vec<Mark>>) -> usize {
    let mut score = 0;

    'line: for line in lines.iter() {
        let mut stack = vec![];
        for mark in line.iter() {
            match mark {
                Mark::Open(chunk) => stack.push(*chunk),
                Mark::Close(chunk) => match stack.pop() {
                    Some(c) => {
                        if c != *chunk {
                            score += chunk.get_invalid_score();
                            continue 'line;
                        }
                    }
                    None => {}
                },
            }
        }
    }

    score
}

fn part_2(lines: &Vec<Vec<Mark>>) -> usize {
    let mut complete_scores = vec![];

    'line: for line in lines.iter() {
        let mut stack = vec![];
        for mark in line.iter() {
            match mark {
                Mark::Open(chunk) => stack.push(*chunk),
                Mark::Close(chunk) => match stack.pop() {
                    Some(c) => {
                        if c != *chunk {
                            // invalid line, skip
                            continue 'line;
                        }
                    }
                    None => {}
                },
            }
        }
        let complete_score = stack
            .iter()
            .rev()
            .map(|c| c.get_autocomplete_score())
            .fold(0, |acc, score| acc * 5 + score);
        complete_scores.push(complete_score);
    }

    complete_scores.sort();
    complete_scores[(complete_scores.len() - 1) / 2]
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let lines = input
        .lines()
        .map(|line| line.chars().map(|c| Mark::from_char(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("Part 1: {}", part_1(&lines));
    println!("Part 2: {}", part_2(&lines));
}
