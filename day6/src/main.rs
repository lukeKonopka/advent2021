use std::{collections::HashMap, fs::read_to_string};

struct LanternfishPool {
    map: HashMap<u8, usize>,
}

impl LanternfishPool {
    fn from_slice(input: &[u8]) -> Self {
        let mut map = HashMap::new();
        for &fish in input.iter() {
            *map.entry(fish).or_default() += 1;
        }

        LanternfishPool { map }
    }

    fn age(&mut self) {
        let mut new_map = HashMap::new();
        for (&age, &count) in self.map.iter() {
            if age == 0 {
                *new_map.entry(6).or_default() += count;
                *new_map.entry(8).or_default() += count;
            } else {
                *new_map.entry(age - 1).or_default() += count;
            }
        }
        self.map = new_map;
    }

    fn count(&self) -> usize {
        self.map.values().sum()
    }
}

fn part_1(input: &[u8]) -> usize {
    let mut pool = LanternfishPool::from_slice(input);
    for _ in 0..80 {
        pool.age();
    }
    pool.count()
}

fn part_2(input: &[u8]) -> usize {
    let mut pool = LanternfishPool::from_slice(input);
    for _ in 0..256 {
        pool.age();
    }
    pool.count()
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");

    let input = input
        .split(',')
        .map(|v| v.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
