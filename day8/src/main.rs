use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    fs::read_to_string,
};

use crate::combinations::combinations;

mod combinations;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

fn is_valid_digit(segments: &[Segment]) -> bool {
    decode_digit(segments).is_some()
}

fn decode_digit(segments: &[Segment]) -> Option<usize> {
    let segments_set: HashSet<&Segment, RandomState> = HashSet::from_iter(segments.iter());
    use Segment::*;
    let valid_combinations = vec![
        (0, vec![A, B, C, E, F, G]),
        (1, vec![C, F]),
        (2, vec![A, C, D, E, G]),
        (3, vec![A, C, D, F, G]),
        (4, vec![B, C, D, F]),
        (5, vec![A, B, D, F, G]),
        (6, vec![A, B, D, E, F, G]),
        (7, vec![A, C, F]),
        (8, vec![A, B, C, D, E, F, G]),
        (9, vec![A, B, C, D, F, G]),
    ];

    for (n, combination) in valid_combinations.iter() {
        let combination_set: HashSet<&Segment, RandomState> =
            HashSet::from_iter(combination.iter());
        if combination_set == segments_set {
            return Some(*n);
        }
    }
    None
}

impl Segment {
    fn from_char(source: char) -> Self {
        match source {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => panic!("Unknown segment {}", source),
        }
    }

    fn all_segments() -> impl Iterator<Item = Segment> {
        ('a'..='g').map(Segment::from_char)
    }
}

type SignalPattern = Vec<Segment>;

struct Entry {
    signal_patterns: Vec<SignalPattern>,
    output_value: Vec<SignalPattern>,
}

impl Entry {
    fn from_str(source: &str) -> Option<Self> {
        let mut entry_iter = source.split(" | ");
        let signal_patterns = entry_iter
            .next()?
            .split(' ')
            .map(|segments| segments.chars().map(Segment::from_char).collect())
            .collect();
        let output_value = entry_iter
            .next()?
            .split(' ')
            .map(|segments| segments.chars().map(Segment::from_char).collect())
            .collect();

        Some(Self {
            signal_patterns,
            output_value,
        })
    }

    fn flatten(&self) -> Vec<SignalPattern> {
        let mut res = vec![];
        for p in self.signal_patterns.iter() {
            res.push(p.clone());
        }
        for p in self.output_value.iter() {
            res.push(p.clone());
        }
        res
    }
}

struct ConnectionMap {
    map: HashMap<Segment, Segment>,
}

impl ConnectionMap {
    fn decode(&self, value: Vec<Segment>) -> Option<usize> {
        let mapped_segments = self.map_entry_value(value);
        decode_digit(&mapped_segments)
    }
    fn map_entry_value(&self, in_value: Vec<Segment>) -> Vec<Segment> {
        in_value
            .iter()
            .map(|in_segment| *self.map.get(in_segment).unwrap())
            .collect()
    }
    fn from_entry(entry: &Entry) -> Self {
        let entry_values = entry.flatten();
        let possible_connections = combinations(Segment::all_segments().collect());

        for possible_connection in possible_connections.into_iter() {
            let possible_map = ConnectionMap::from_vec_tuple(possible_connection);
            if entry_values
                .iter()
                .map(|entry_value| possible_map.map_entry_value(entry_value.clone()))
                .all(|a| is_valid_digit(&a))
            {
                return possible_map;
            }
        }

        panic!("No possible connection map found for entry");
    }

    fn from_vec_tuple(source: Vec<(Segment, Segment)>) -> Self {
        let mut map = HashMap::new();

        for (from, to) in source.iter() {
            map.insert(*from, *to);
        }

        ConnectionMap { map }
    }
}

fn part_1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|entry| entry.output_value.iter())
        .filter(|segments| match segments.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn part_2(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|e| {
            let map = ConnectionMap::from_entry(e);
            let digits = e
                .output_value
                .iter()
                .map(|d| format!("{}", map.decode(d.clone()).unwrap()))
                .collect::<Vec<_>>()
                .join("");
            usize::from_str_radix(&digits, 10).unwrap()
        })
        .sum()
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let entries = input
        .lines()
        .map(Entry::from_str)
        .map(Option::unwrap)
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&entries));
    println!("Part 2: {}", part_2(&entries));
}
