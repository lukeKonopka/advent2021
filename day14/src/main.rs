use std::{collections::HashMap, fs::read_to_string, iter};

#[derive(Clone, Copy)]
struct Rule {
    pair: (char, char),
    insert: char,
}

impl Rule {
    fn from_str(value: &str) -> Self {
        let mut it = value.split(" -> ");
        let pair = it.next().unwrap();
        let insert = it.next().unwrap().chars().next().unwrap();
        let mut it = pair.chars();
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        Self {
            pair: (a, b),
            insert,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct PolymerLink(char, Option<char>);

impl From<Rule> for PolymerLink {
    fn from(rule: Rule) -> Self {
        Self(rule.pair.0, Some(rule.pair.1))
    }
}

#[derive(Clone)]
struct Polymer {
    map: HashMap<PolymerLink, usize>,
    rules: Vec<Rule>,
}

impl Polymer {
    fn from_template(template: &str, rules: Vec<Rule>) -> Self {
        let mut map = HashMap::new();
        let chars = template.chars();
        let chars_offset = template
            .chars()
            .skip(1)
            .map(|v| Some(v))
            .chain(iter::repeat(None));
        for (a, b) in chars.zip(chars_offset) {
            map.entry(PolymerLink(a, b))
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        Self { map, rules }
    }

    fn step(&mut self) {
        let mut new_map = HashMap::new();
        for rule in self.rules.iter() {
            let matching_link = (*rule).into();
            if let Some(matched_count) = self.map.get(&matching_link).copied() {
                let before = PolymerLink(matching_link.0, Some(rule.insert));
                let after = PolymerLink(rule.insert, matching_link.1);

                new_map
                    .entry(before)
                    .and_modify(|count| *count += matched_count)
                    .or_insert(matched_count);
                new_map
                    .entry(after)
                    .and_modify(|count| *count += matched_count)
                    .or_insert(matched_count);
            }
        }

        for (k, v) in self.map.iter() {
            if self
                .rules
                .iter()
                .find(|&r| PolymerLink::from(*r) == *k)
                .is_none()
            {
                new_map.entry(*k).or_insert(*v);
            }
        }

        self.map = new_map;
    }

    fn get_count(&self) -> HashMap<char, usize> {
        let mut map = HashMap::new();
        for (c, count) in self.map.iter() {
            map.entry(c.0)
                .and_modify(|v| *v += *count)
                .or_insert(*count);
        }
        map
    }
}

fn part_1(polymer: &Polymer) -> usize {
    let mut polymer = polymer.clone();
    for _ in 0..10 {
        polymer.step();
    }
    let most = polymer.get_count().iter().map(|(_, c)| *c).max().unwrap();
    let least = polymer.get_count().iter().map(|(_, c)| *c).min().unwrap();
    most - least
}

fn part_2(polymer: &Polymer) -> usize {
    let mut polymer = polymer.clone();
    for _ in 0..40 {
        polymer.step();
    }
    let most = polymer.get_count().iter().map(|(_, c)| *c).max().unwrap();
    let least = polymer.get_count().iter().map(|(_, c)| *c).min().unwrap();
    most - least
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let mut lines = input.lines();
    let template = lines.next().unwrap();
    let rules = lines.skip(1).map(Rule::from_str).collect();
    let polymer = Polymer::from_template(template, rules);

    println!("Part 1: {}", part_1(&polymer));
    println!("Part 2: {}", part_2(&polymer));
}
