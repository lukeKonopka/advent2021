use std::fs::read_to_string;

const GRID_SIZE: usize = 10;

struct Grid {
    flashed: Vec<usize>,
    flash_count: usize,
    values: [u8; GRID_SIZE * GRID_SIZE],
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let values_vec = input
            .chars()
            .filter_map(|c| {
                if c.is_digit(10) {
                    format!("{}", c).parse::<u8>().ok()
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut values = [0; GRID_SIZE * GRID_SIZE];
        for (idx, v) in values_vec.into_iter().enumerate() {
            values[idx] = v;
        }

        Self {
            flashed: vec![],
            values,
            flash_count: 0,
        }
    }

    fn flash(&mut self, x: usize, y: usize) {
        let index = y as usize * GRID_SIZE + x as usize;
        self.flash_count += 1;
        self.flashed.push(index);

        let adjecent_diffs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let adjecent = adjecent_diffs
            .iter()
            .map(|(diff_x, diff_y)| (x as isize + diff_x, y as isize + diff_y))
            .collect::<Vec<_>>();

        for (adjecent_x, adjecent_y) in adjecent.iter() {
            if let Some(octopus_value) = self.get_mut(*adjecent_x, *adjecent_y) {
                if *octopus_value <= 9 {
                    *octopus_value += 1;
                }
            }
        }
    }

    fn has_pending_flashes(&self) -> bool {
        self.values
            .iter()
            .enumerate()
            .any(|(idx, v)| *v > 9 && !self.flashed.contains(&idx))
    }

    fn step(&mut self) {
        // reset flashed
        self.flashed = vec![];

        // increase all octopuses
        for v in self.values.iter_mut() {
            *v += 1;
        }

        // flash until no octopus is triggered
        while self.has_pending_flashes() {
            for (idx, v) in self.values.clone().iter().enumerate() {
                if *v > 9 && !self.flashed.contains(&idx) {
                    let x = idx % GRID_SIZE;
                    let y = idx / GRID_SIZE;
                    self.flash(x, y);
                }
            }
        }

        // reset all that flashed
        for flashed_idx in self.flashed.iter() {
            self.values[*flashed_idx] = 0;
        }
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut u8> {
        if x >= 0 && y >= 0 && x < GRID_SIZE as isize && y < GRID_SIZE as isize {
            let x = x as usize;
            let y = y as usize;
            let index = y * GRID_SIZE + x;
            Some(&mut self.values[index])
        } else {
            None
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut grid = Grid::from_str(input);
    for _ in 0..100 {
        grid.step();
    }
    grid.flash_count
}

fn part_2(input: &str) -> usize {
    let mut grid = Grid::from_str(input);
    let mut step_n = 1;
    loop {
        grid.step();
        if (0..GRID_SIZE * GRID_SIZE).all(|idx| grid.flashed.contains(&idx)) {
            return step_n;
        }
        step_n += 1;
    }
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
