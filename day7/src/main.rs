use std::fs::read_to_string;

fn find_cheapest_position(positions: &[usize], cost_fn: fn(usize) -> usize) -> usize {
    positions
        .iter()
        .map(|&i| {
            positions
                .iter()
                .map(|&crab| (i as isize - crab as isize).abs() as usize)
                .map(|dist| cost_fn(dist))
                .sum()
        })
        .min()
        .unwrap()
}

fn incremental_cost(distance: usize) -> usize {
    (0..=distance).sum()
}

fn part_1(crabs: &[usize]) -> usize {
    find_cheapest_position(crabs, std::convert::identity)
}

fn part_2(crabs: &[usize]) -> usize {
    find_cheapest_position(crabs, incremental_cost)
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let crabs = input
        .split(",")
        .map(|crab| crab.parse::<usize>().expect("Cannot parse crab value"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&crabs));
    println!("Part 2: {}", part_2(&crabs));
}
