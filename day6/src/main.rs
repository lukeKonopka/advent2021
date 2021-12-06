use std::{collections::HashMap, fs::read_to_string};

struct LanternfishPool {
    map: HashMap<u8, usize>,
}

impl LanternfishPool {
    fn from_list(input: &[u8]) -> Self {
        let mut map = HashMap::new();
        for &fish in input.iter() {
            let entry = map.entry(fish).or_insert(0);
            *entry += 1;
        }

        LanternfishPool { map }
    }

    fn age(&mut self) {
        let mut new_map = HashMap::new();
        for (&age, &count) in self.map.iter() {
            if age == 0 {
                // those who gave birth
                let new_age = 6;
                let entry = new_map.entry(new_age).or_insert(0);
                *entry += count;

                // new ones
                let new_age = 8;
                let entry = new_map.entry(new_age).or_insert(0);
                *entry += count;
            } else {
                let new_age = age - 1;
                let entry = new_map.entry(new_age).or_insert(0);
                *entry += count;
            }
        }
        self.map = new_map;
    }

    fn count(&self) -> usize {
        self.map.values().sum()
    }
}

fn part_1(input: &[u8]) -> usize {
    let mut pool = LanternfishPool::from_list(input);
    for _ in 0..80 {
        pool.age();
    }
    pool.count()
}

fn part_2(input: &[u8]) -> usize {
    let mut pool = LanternfishPool::from_list(input);
    for _ in 0..256 {
        pool.age();
    }
    pool.count()
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");

    let input = input
        .split(',')
        .map(|v| v.parse::<u8>().expect("Cannot parse usize"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
