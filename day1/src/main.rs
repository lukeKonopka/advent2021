use std::fs::read_to_string;

fn zip_3<T, U, V>(
    a: impl Iterator<Item = T>,
    b: impl Iterator<Item = U>,
    c: impl Iterator<Item = V>,
) -> impl Iterator<Item = (T, U, V)> {
    a.zip(b).zip(c).map(|((a, b), c)| (a, b, c))
}

fn part_1(depths: &[u32]) -> u32 {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .map(|(&a, &b)| if b > a { 1 } else { 0 })
        .sum()
}

fn part_2(depths: &[u32]) -> u32 {
    let windows = zip_3(depths.iter(), depths.iter().skip(1), depths.iter().skip(2))
        .map(|(a, b, c)| a + b + c)
        .collect::<Vec<_>>();
    part_1(&windows)
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input");
    let input_lines = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input_lines));
    println!("Part 2: {}", part_2(&input_lines));
}
