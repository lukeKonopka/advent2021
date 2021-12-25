use model::{BasinPoint, HeightMap, Neighbours};
use std::{cell::RefCell, collections::HashMap, fs::read_to_string};

mod model;

fn group<T>(i: impl Iterator<Item = T>) -> HashMap<T, Vec<T>>
where
    T: core::hash::Hash + Clone + Copy + Eq,
{
    let mut map = HashMap::new();
    for item in i {
        let entry = map.entry(item);
        entry
            .and_modify(|items: &mut Vec<T>| items.push(item))
            .or_insert(vec![item]);
    }
    map
}

fn part_1(map: &HeightMap<usize>) -> usize {
    map.map_neighbours(&|n: Neighbours<usize>| {
        if n.is_low_point() {
            Some(n.value)
        } else {
            None
        }
    })
    .values()
    .map(|v| v.map(|v| v + 1).unwrap_or(0))
    .sum()
}

fn part_2(map: &HeightMap<usize>) -> usize {
    let next_id = RefCell::new(0);

    // mark initial basins
    let mut basin_map = map.map_neighbours(&|n: Neighbours<usize>| {
        if n.is_low_point() {
            let mut next_idx = next_id.borrow_mut();
            let basin_id = *next_idx;
            *next_idx = basin_id + 1;
            BasinPoint::Marked { basin_id }
        } else {
            if n.value == 9 {
                BasinPoint::Ridge
            } else {
                BasinPoint::Unmarked
            }
        }
    });

    // grow marked basins
    while !basin_map.values().all(|p| p.is_marked() || p.is_ridge()) {
        let new_basin_map = basin_map.map_neighbours(&|n| {
            let marked = [n.top, n.left, n.right, n.bottom]
                .iter()
                .find_map(|&point| point.filter(|p| p.is_marked()));

            match marked {
                Some(BasinPoint::Marked { basin_id, .. }) => n.value.mark(basin_id),
                _ => n.value,
            }
        });
        basin_map = new_basin_map;
    }

    // find largest basins
    let basin_ids = basin_map.values().filter_map(|p| match p {
        BasinPoint::Marked { basin_id, .. } => Some(basin_id),
        _ => None,
    });
    let grouped = group(basin_ids);
    let mut basins = grouped
        .iter()
        .map(|(&id, v)| (*id, v.len()))
        .collect::<Vec<_>>();
    basins.sort_by_key(|(_, l)| *l);

    let mut biggest = basins.iter().rev();
    let a = biggest.next().unwrap().1;
    let b = biggest.next().unwrap().1;
    let c = biggest.next().unwrap().1;

    a * b * c
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let height_map = HeightMap::from_str(&input);

    println!("Part 1: {}", part_1(&height_map));
    println!("Part 2: {}", part_2(&height_map));
}
