use std::{collections::VecDeque, fmt::Display, fs::read_to_string};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct NodeId(usize);

#[derive(Clone, Copy, Debug)]
struct NodeInfo {
    id: NodeId,
    cost: Option<usize>,
    previous_id: Option<NodeId>,
}

struct Map {
    values: Vec<usize>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let height = s.lines().count();
        let width = s.lines().next().expect("Input has no lines").len();
        let values = s
            .chars()
            .filter_map(|c| format!("{}", c).parse::<usize>().ok())
            .collect();
        Self {
            values,
            width,
            height,
        }
    }

    fn set(&mut self, x: usize, y: usize, value: usize) {
        let index = y * self.width + x;
        self.values[index] = value;
    }

    fn get(&self, x: usize, y: usize) -> usize {
        let index = y * self.width + x;
        self.values[index]
    }

    fn enlarged(&self) -> Self {
        let new_width = self.width * 5;
        let new_height = self.height * 5;
        let mut new_map = Map {
            values: vec![0; new_width * new_height],
            width: new_width,
            height: new_height,
        };

        for i in 0..25 {
            for y in 0..self.width {
                for x in 0..self.height {
                    let tile_x = i % 5;
                    let tile_y = i / 5;
                    let dist = tile_x + tile_y;
                    let value = self.get(x, y);
                    let new_value = if value + dist > 9 {
                        ((value + dist) % 10) + 1
                    } else {
                        value + dist
                    };
                    new_map.set(x + self.width * tile_x, y + self.height * tile_y, new_value);
                }
            }
        }

        new_map
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                write!(f, "{}  ", self.values[index])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct DijkstraWalker<'a> {
    map: &'a Map,
    priority_queue: VecDeque<NodeInfo>,
    visited: Vec<NodeInfo>,
}

impl<'a> DijkstraWalker<'a> {
    fn from_map(map: &'a Map) -> Self {
        let priority_queue = map
            .values
            .iter()
            .enumerate()
            .map(|(id, _)| NodeInfo {
                id: NodeId(id),
                cost: None,
                previous_id: None,
            })
            .collect();

        Self {
            map,
            priority_queue,
            visited: vec![],
        }
    }

    fn set_node_cost(&mut self, id: NodeId, cost: usize) {
        let node = self
            .priority_queue
            .iter_mut()
            .find(|node_info| node_info.id == id)
            .expect("Attempting to set cost of node that doesn't exist in priority queue");
        (*node).cost = Some(cost);
    }

    fn sort_priority(&mut self) {
        let mut v = Vec::from(self.priority_queue.clone());
        v.sort_by_key(|NodeInfo { cost, .. }| cost.unwrap_or(usize::MAX));
        self.priority_queue = VecDeque::from(v);
    }

    fn set_node_prev(&mut self, id: NodeId, prev_id: NodeId) {
        let node = self
            .priority_queue
            .iter_mut()
            .find(|node_info| node_info.id == id)
            .expect("Attempting to set previous id of node that doesn't exist in priority queue");

        (*node).previous_id = Some(prev_id);
    }

    fn neighbors(&self, id: NodeId) -> Vec<NodeId> {
        let x = id.0 % self.map.width;
        let id = id.0 as isize;

        let up = id - self.map.width as isize;
        let down = id + self.map.width as isize;
        let left = if x > 0 { id - 1 } else { -1 };
        let right = if x < self.map.width - 1 { id + 1 } else { -1 };

        [up, down, left, right]
            .iter()
            .filter(|v| {
                **v >= 0
                    && **v < self.map.values.len() as isize
                    && self
                        .priority_queue
                        .iter()
                        .find(|a| a.id.0 == **v as usize)
                        .is_some()
            })
            .map(|v| NodeId(*v as usize))
            .collect()
    }

    fn search(&mut self, start_id: NodeId, end_id: NodeId) {
        self.set_node_cost(start_id, 0);
        self.sort_priority();
        let mut curr = self.priority_queue[0];

        while curr.id != end_id {
            let neighbor_ids = self.neighbors(curr.id);
            for neighbor_id in neighbor_ids.iter() {
                let move_cost = self.map.values[neighbor_id.0];
                let current_cost = curr
                    .cost
                    .expect("Current node should have cost calculated already");
                let neigbor_calc_cost = self
                    .priority_queue
                    .iter()
                    .find(|info| info.id == *neighbor_id)
                    .and_then(|i| i.cost);

                let new_cost = move_cost + current_cost;

                if let Some(old_cost) = neigbor_calc_cost {
                    // if already evaluated, check if new evaluation is cheaper
                    if new_cost < old_cost {
                        self.set_node_prev(*neighbor_id, curr.id);
                        self.set_node_cost(*neighbor_id, new_cost);
                    }
                } else {
                    self.set_node_prev(*neighbor_id, curr.id);
                    self.set_node_cost(*neighbor_id, new_cost);
                }
            }

            self.sort_priority();

            curr = self.priority_queue[0];
            self.visited.push(self.priority_queue.pop_front().unwrap());
        }
    }

    fn get_path(&self, end_id: NodeId) -> Vec<(NodeId, usize)> {
        let mut current_node = self.visited.iter().find(|v| v.id == end_id).unwrap();
        let mut path = vec![];

        while current_node.previous_id.is_some() {
            let previous_node = self
                .visited
                .iter()
                .find(|n| n.id == current_node.previous_id.unwrap())
                .unwrap();
            path.push((current_node.id, self.map.values[current_node.id.0]));
            current_node = previous_node;
        }

        path.into_iter().rev().collect()
    }
}
fn part_1(map: &Map) -> usize {
    let mut walker = DijkstraWalker::from_map(map);
    walker.search(NodeId(0), NodeId(map.values.len() - 1));
    walker
        .get_path(NodeId(map.values.len() - 1))
        .iter()
        .map(|a| a.1)
        .sum::<usize>()
}

fn part_2(map: &Map) -> usize {
    let larger_map = map.enlarged();
    let mut walker = DijkstraWalker::from_map(&larger_map);
    walker.search(NodeId(0), NodeId(map.values.len() - 1));
    walker
        .get_path(NodeId(map.values.len() - 1))
        .iter()
        .map(|a| a.1)
        .sum::<usize>()
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let map = Map::from_str(&input);

    println!("Part 1: {}", part_1(&map));
    println!("Part 2: {}", part_2(&map));
}
