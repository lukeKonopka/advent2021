use std::fs::read_to_string;

trait FromBinaryDigits {
    fn as_binary_number(&self) -> usize;
}

impl FromBinaryDigits for Vec<u8> {
    fn as_binary_number(&self) -> usize {
        let str_value = self
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join("");
        usize::from_str_radix(&str_value, 2).expect("cannot convert vec of u8 to usize")
    }
}

#[derive(Debug)]
struct InputMatrix {
    values: Vec<Vec<u8>>,
    row_len: usize,
}

impl InputMatrix {
    fn from_slice(source: &Vec<&Vec<u8>>) -> Self {
        let values = source.iter().map(|&a| a.clone()).collect::<Vec<_>>();
        let row_len = values.get(0).expect("Empty input").len();
        InputMatrix { values, row_len }
    }

    fn from_str(source: &str) -> Self {
        let values = source
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| String::from(c).parse::<u8>().expect("Cannot parse"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let row_len = values.get(0).expect("Empty input").len();
        InputMatrix { values, row_len }
    }

    fn transpose(&self) -> Self {
        let mut values = vec![];
        for col_idx in 0..self.row_len {
            let mut new_row = vec![];
            for row_idx in 0..self.values.len() {
                new_row.push(self.values[row_idx][col_idx]);
            }
            values.push(new_row);
        }
        let row_len = values.get(0).expect("Empty input").len();
        InputMatrix { values, row_len }
    }
}

fn count_ones_zeros(values: &[u8]) -> (usize, usize) {
    let (zeros, ones): (Vec<&u8>, Vec<&u8>) = values.iter().partition(|&v| *v == 0);
    (zeros.len(), ones.len())
}

fn part_1(input_matrix: &InputMatrix) -> usize {
    let transposed = input_matrix.transpose();
    let (gamma_digits, epsilon_digits): (Vec<u8>, Vec<u8>) = transposed
        .values
        .iter()
        .map(|column| {
            let (zeros_count, ones_count) = count_ones_zeros(&column);
            let gamma_digit: u8 = if zeros_count > ones_count { 0 } else { 1 };
            let epsilon_digit: u8 = if zeros_count > ones_count { 1 } else { 0 };
            (gamma_digit, epsilon_digit)
        })
        .unzip();

    gamma_digits.as_binary_number() * epsilon_digits.as_binary_number()
}

fn decode_value(list: Vec<&Vec<u8>>, bit_criteria_fn: fn(usize, usize, u8) -> bool) -> usize {
    let mut candidate_list = list;
    let mut current_digit_idx = 0;
    while candidate_list.len() > 1 {
        let mut new_candidate_list = vec![];
        for candidate in candidate_list.iter() {
            let candidate_digit = candidate[current_digit_idx];
            let candidate_matrix = InputMatrix::from_slice(&candidate_list);
            let values = candidate_matrix.transpose().values[current_digit_idx].clone();
            let (zeros_count, ones_count) = count_ones_zeros(&values);

            if bit_criteria_fn(zeros_count, ones_count, candidate_digit) {
                new_candidate_list.push(candidate.clone());
            }
        }
        candidate_list = new_candidate_list;
        current_digit_idx += 1;
    }

    candidate_list
        .get(0)
        .map(|n| n.as_binary_number())
        .expect("Cannot decode value")
}

fn oxygen_bit_criteria(zero_count: usize, one_count: usize, current_value: u8) -> bool {
    use std::cmp::Ordering::*;
    let keep_zero = current_value == 0;
    let keep_one = current_value == 1;

    match zero_count.cmp(&one_count) {
        Less => keep_one,
        Equal => keep_one,
        Greater => keep_zero,
    }
}

fn co2_bit_criteria(zero_count: usize, one_count: usize, current_value: u8) -> bool {
    use std::cmp::Ordering::*;
    let keep_zero = current_value == 0;
    let keep_one = current_value == 1;

    match zero_count.cmp(&one_count) {
        Less => keep_zero,
        Equal => keep_zero,
        Greater => keep_one,
    }
}

fn part_2(input_matrix: &InputMatrix) -> usize {
    let initial = input_matrix.values.iter().collect::<Vec<_>>();
    let oxygen = decode_value(initial.clone(), oxygen_bit_criteria);
    let co2 = decode_value(initial.clone(), co2_bit_criteria);

    oxygen * co2
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let input_matrix = InputMatrix::from_str(&input);

    println!("Part 1: {}", part_1(&input_matrix));
    println!("Part 2: {}", part_2(&input_matrix));
}
