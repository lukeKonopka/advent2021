use crate::Segment;

fn _combinations(
    from_list: Vec<Segment>,
    to_list: Vec<Segment>,
    acc: Vec<(Segment, Segment)>,
) -> Vec<Vec<(Segment, Segment)>> {
    match from_list.first() {
        Some(from) => {
            let mut r = vec![];
            for to in to_list.iter() {
                let from_list = from_list
                    .iter()
                    .filter(|&e| e != from)
                    .copied()
                    .collect::<Vec<_>>();
                let to_list = to_list
                    .iter()
                    .filter(|&e| e != to)
                    .copied()
                    .collect::<Vec<_>>();
                let foo_r = _combinations(
                    from_list,
                    to_list,
                    vec![acc.clone(), vec![(*from, *to)]].concat(),
                );
                r = vec![r, foo_r].concat();
            }
            r
        }
        None => {
            vec![acc]
        }
    }
}

pub fn combinations(list: Vec<Segment>) -> Vec<Vec<(Segment, Segment)>> {
    _combinations(list.clone(), list.clone(), vec![])
}
