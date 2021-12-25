pub struct Neighbours<T> {
    pub value: T,
    pub top: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,
    pub bottom: Option<T>,
}

impl<T: Ord + Clone + Copy> Neighbours<T> {
    pub fn is_low_point(&self) -> bool {
        [self.top, self.left, self.right, self.bottom]
            .iter()
            .all(|&v| v.map(|v| v > self.value).unwrap_or(true))
    }
}

pub struct HeightMap<T> {
    points: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: Clone + Copy> HeightMap<T> {
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.points.iter().flat_map(|r| r.iter())
    }

    pub fn map_neighbours<U>(&self, map_fn: &dyn Fn(Neighbours<T>) -> U) -> HeightMap<U> {
        let mut new_points = vec![];
        for row_idx in 0..self.height {
            let mut new_row = vec![];
            for col_idx in 0..self.width {
                let point_neighbours = self.get_neighbours(row_idx, col_idx);
                let new_point = map_fn(point_neighbours);
                new_row.push(new_point);
            }
            new_points.push(new_row);
        }

        HeightMap {
            points: new_points,
            width: self.width,
            height: self.height,
        }
    }

    fn get_neighbours(&self, row_idx: usize, col_idx: usize) -> Neighbours<T> {
        let row_idx = row_idx as isize;
        let col_idx = col_idx as isize;
        Neighbours {
            value: self.get(row_idx, col_idx).unwrap(),
            top: self.get(row_idx - 1, col_idx),
            left: self.get(row_idx, col_idx - 1),
            right: self.get(row_idx, col_idx + 1),
            bottom: self.get(row_idx + 1, col_idx),
        }
    }

    fn get(&self, row_idx: isize, col_idx: isize) -> Option<T> {
        if row_idx < 0
            || col_idx < 0
            || row_idx >= self.height as isize
            || col_idx >= self.width as isize
        {
            None
        } else {
            Some(self.points[row_idx as usize][col_idx as usize])
        }
    }
}

impl HeightMap<usize> {
    pub fn from_str(input: &str) -> Self {
        let points: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| format!("{}", c).parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        let width = points[0].len();
        let height = points.len();

        Self {
            points,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy)]
pub enum BasinPoint {
    Marked { basin_id: usize },
    Unmarked,
    Ridge,
}

impl BasinPoint {
    pub fn mark(&self, basin_id: usize) -> Self {
        match &self {
            BasinPoint::Unmarked { .. } => BasinPoint::Marked { basin_id },
            _ => *self,
        }
    }

    pub fn is_marked(&self) -> bool {
        match self {
            BasinPoint::Marked { .. } => true,
            _ => false,
        }
    }

    pub fn is_ridge(&self) -> bool {
        match self {
            BasinPoint::Ridge => true,
            _ => false,
        }
    }
}
