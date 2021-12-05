use std::fs::read_to_string;

use bingo::BingoBoard;

mod bingo;

#[derive(Clone, Debug)]
struct Input {
    drawn_numbers: Vec<u8>,
    boards: Vec<BingoBoard>,
}

impl Input {
    fn from_str(input: &str) -> Self {
        let mut groups = input.split("\n\n");
        let drawn_numbers = groups
            .next()
            .expect("No drawn numbers list")
            .split(',')
            .map(|n| n.parse::<u8>().expect("cannot parse value as u8"))
            .collect();

        let boards = groups.map(BingoBoard::from_str).collect();

        Self {
            drawn_numbers,
            boards,
        }
    }
}

fn part_1(input: &Input) -> usize {
    let mut input = input.clone();
    for drawn_number in input.drawn_numbers {
        for board in input.boards.iter_mut() {
            board.mark_number(drawn_number);
            if board.is_winning() {
                let unmarked_sum: usize =
                    board.unmarked_iter().map(|v| v.get_number() as usize).sum();
                return unmarked_sum * drawn_number as usize;
            }
        }
    }

    panic!("No board won")
}

fn part_2(input: &Input) -> usize {
    let mut input = input.clone();

    let mut last_winning_board = Option::None;
    let mut last_winning_number = Option::None;
    for drawn_number in input.drawn_numbers {
        for board in input.boards.iter_mut() {
            if !board.is_winning() {
                board.mark_number(drawn_number);
                if board.is_winning() {
                    last_winning_board = Option::Some(board.clone());
                    last_winning_number = Option::Some(drawn_number);
                }
            }
        }
    }

    match (last_winning_board, last_winning_number) {
        (Some(b), Some(n)) => {
            let unmarked_sum: usize = b.unmarked_iter().map(|v| v.get_number() as usize).sum();
            unmarked_sum * n as usize
        }
        _ => panic!("No board won"),
    }
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let input = Input::from_str(&input);
    println!("Part 1: {}", part_1(&input));
    println!("Part 1: {}", part_2(&input));
}
