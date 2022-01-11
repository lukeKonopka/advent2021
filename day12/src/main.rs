use std::fs::read_to_string;

struct Vertex {
    id: String,
    adjacent: Vec<String>,
}

struct Graph {
    vertices: Vec<Vertex>,
}

impl Graph {
    fn from_edge_list(edges: Vec<(String, String)>) -> Self {
        let mut vertices = vec![];

        for (from_id, to_id) in edges.iter() {
            if vertices
                .iter()
                .find(|vertex: &&Vertex| &vertex.id == from_id)
                .is_none()
            {
                vertices.push(Vertex {
                    id: from_id.clone(),
                    adjacent: vec![],
                });
            }
            let from_vert = vertices.iter_mut().find(|v| &v.id == from_id).unwrap();
            if !from_vert.adjacent.contains(&to_id) {
                from_vert.adjacent.push(to_id.clone());
            }

            if vertices.iter().find(|v| &v.id == to_id).is_none() {
                vertices.push(Vertex {
                    id: to_id.clone(),
                    adjacent: vec![],
                });
            }
            let to_vert = vertices.iter_mut().find(|v| &v.id == to_id).unwrap();
            if !to_vert.adjacent.contains(&from_id) {
                to_vert.adjacent.push(from_id.clone());
            }
        }

        Self { vertices }
    }

    fn possible_paths<'a>(
        &'a self,
        start: &'a str,
        end: &'a str,
        path: Vec<&'a str>,
        qualify_fn: fn(&str, Vec<&str>) -> bool,
    ) -> Vec<Vec<&'a str>> {
        if start == end {
            return vec![vec![path.clone(), vec![end]].concat()];
        }
        let start_vert = self
            .vertices
            .iter()
            .find(|v| v.id == start)
            .expect("Cannot find start vertex");

        start_vert
            .adjacent
            .iter()
            .map(|adj| {
                if qualify_fn(start, path.clone()) {
                    self.possible_paths(
                        adj,
                        end,
                        vec![path.clone(), vec![start]].concat(),
                        qualify_fn,
                    )
                } else {
                    vec![]
                }
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

fn is_large(id: &str) -> bool {
    id.chars().all(|c| c.is_uppercase())
}

fn only_once(curr: &str, path: Vec<&str>) -> bool {
    is_large(curr) || !path.contains(&curr)
}

fn part_1(graph: &Graph) -> usize {
    graph
        .possible_paths("start", "end", vec![], only_once)
        .len()
}

fn part_2(graph: &Graph) -> usize {
    graph
        .possible_paths("start", "end", vec![], |curr, path| {
            // if path contains start/end and we encounter duplicate of it - reject
            if path.contains(&"start") && curr == "start" || path.contains(&"end") && curr == "end"
            {
                return false;
            }

            let contains_duplicate = path.iter().filter(|v| !is_large(v)).any(|v| {
                let dups = path.iter().filter(|a| a == &v).collect::<Vec<_>>();
                dups.len() > 1
            });

            !contains_duplicate || only_once(curr, path)
        })
        .len()
}

fn main() {
    let edges = read_to_string("./input")
        .expect("Cannot read input file")
        .lines()
        .map(|l| {
            let mut it = l.split("-");
            let from = it.next().unwrap();
            let to = it.next().unwrap();
            (String::from(from), String::from(to))
        })
        .collect();
    let graph = Graph::from_edge_list(edges);

    println!("Part 1: {}", part_1(&graph));
    println!("Part 2: {}", part_2(&graph));
}
