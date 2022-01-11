use std::{collections::HashSet, fmt::Display, fs::read_to_string, iter::FromIterator};

#[derive(Clone, Copy)]
enum Instruction {
    FoldUp(usize),
    FoldLeft(usize),
}

impl Instruction {
    fn from_str(value: &str) -> Self {
        let mut it = value.split("=");
        let cmd = it.next().unwrap();
        let value = it.next().and_then(|v| v.parse::<usize>().ok()).unwrap();
        if cmd.contains("x") {
            Self::FoldLeft(value)
        } else if cmd.contains("y") {
            Self::FoldUp(value)
        } else {
            panic!("Cannot parse {}", value);
        }
    }

    fn execute(&self, paper: &mut Paper) {
        match self {
            Instruction::FoldUp(y) => paper.fold_up(*y),
            Instruction::FoldLeft(x) => paper.fold_left(*x),
        }
    }
}

#[derive(Clone)]
struct Paper {
    dots: Vec<(isize, isize)>,
}

impl Paper {
    fn from_input(input: &str) -> Self {
        let dots = input
            .lines()
            .map(|line| {
                let mut it = line.split(",");
                let x = it.next().and_then(|num| num.parse::<isize>().ok()).unwrap();
                let y = it.next().and_then(|num| num.parse::<isize>().ok()).unwrap();
                (x, y)
            })
            .collect();

        Self { dots }
    }

    fn fold_up(&mut self, fold_y: usize) {
        let fold_y = fold_y as isize;
        for (_, dot_y) in self.dots.iter_mut() {
            if *dot_y > fold_y {
                let diff = *dot_y - fold_y;
                *dot_y = fold_y - diff;
            }
        }
    }

    fn fold_left(&mut self, fold_x: usize) {
        let fold_x = fold_x as isize;
        for (dot_x, _) in self.dots.iter_mut() {
            if *dot_x > fold_x {
                let diff = *dot_x - fold_x;
                *dot_x = fold_x - diff;
            }
        }
    }

    fn dots_count(&self) -> usize {
        let set: HashSet<&(isize, isize)> = HashSet::from_iter(self.dots.iter());
        set.len()
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().map(|(x, _)| x).max().unwrap();
        let max_y = self.dots.iter().map(|(_, y)| y).max().unwrap();
        for y in 0..=*max_y {
            for x in 0..=*max_x {
                match self.dots.iter().find(|&&dot| dot == (x, y)) {
                    Some(_) => write!(f, "█")?,
                    None => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_1(paper: &Paper, instructions: &[Instruction]) -> usize {
    let first_instruction = instructions[0];
    let mut paper = paper.clone();
    first_instruction.execute(&mut paper);
    paper.dots_count()
}

fn part_2(paper: &Paper, instructions: &[Instruction]) -> String {
    let mut paper = paper.clone();
    for instruction in instructions.iter() {
        instruction.execute(&mut paper);
    }

    format!("\n{}", paper)
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let mut input_iter = input.split("\n\n");
    let paper = Paper::from_input(input_iter.next().unwrap());
    let instructions = input_iter
        .next()
        .unwrap()
        .lines()
        .map(Instruction::from_str)
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&paper, &instructions));
    println!("Part 2: {}", part_2(&paper, &instructions));
}
