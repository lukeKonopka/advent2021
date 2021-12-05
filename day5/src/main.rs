use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_str(source: &str) -> Point {
        let mut coords = source
            .split(',')
            .map(|v| v.parse::<usize>().expect("cannot parse as usize"));
        let x = coords.next().expect("Cannot read point x coord");
        let y = coords.next().expect("Cennot read point y coord");
        Point { x, y }
    }
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

struct PointsIterator {
    curr: Point,
    len: isize,
    x_step: isize,
    y_step: isize,
}

impl Iterator for PointsIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len < 0 {
            None
        } else {
            let current = self.curr;

            self.len -= 1;
            self.curr = Point {
                x: (self.curr.x as isize + self.x_step) as usize,
                y: (self.curr.y as isize + self.y_step) as usize,
            };

            Some(current)
        }
    }
}

impl Line {
    fn from_str(source: &str) -> Line {
        let mut line_iter = source.split(" -> ").map(Point::from_str);
        let from = line_iter.next().expect("Cannot read line starting point");
        let to = line_iter.next().expect("Cennot read line ending point");

        Line { from, to }
    }

    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    fn points_iter(&self) -> impl Iterator<Item = Point> {
        use std::cmp::Ordering::*;
        let find_diff = |a: usize, b: usize| match a.cmp(&b) {
            Less => 1,
            Equal => 0,
            Greater => -1,
        };
        let x_step = find_diff(self.from.x, self.to.x);
        let y_step = find_diff(self.from.y, self.to.y);
        let len = (self.from.x as isize - self.to.x as isize)
            .abs()
            .max((self.from.y as isize - self.to.y as isize).abs());

        PointsIterator {
            curr: self.from,
            len,
            x_step,
            y_step,
        }
    }
}

#[derive(Debug)]
struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn from_str(input: &str) -> Self {
        let lines = input.lines().map(Line::from_str).collect();
        Input { lines }
    }

    fn width(&self) -> usize {
        self.lines
            .iter()
            .map(|line| line.from.x.max(line.to.x))
            .max()
            .unwrap()
            + 1
    }

    fn height(&self) -> usize {
        self.lines
            .iter()
            .map(|line| line.from.y.max(line.to.y))
            .max()
            .unwrap()
            + 1
    }
}

struct Map {
    values: Vec<Vec<usize>>,
}

impl Map {
    fn with_size(width: usize, height: usize) -> Self {
        let mut values = vec![];
        let mut row = vec![];
        row.resize(width, 0);
        values.resize(height, row);

        Map { values }
    }
}

fn part_1(input: &Input) -> usize {
    let mut map = Map::with_size(input.width(), input.height());

    for line in input.lines.iter() {
        if line.is_horizontal() || line.is_vertical() {
            for line_point in line.points_iter() {
                map.values[line_point.y][line_point.x] += 1;
            }
        }
    }

    map.values.iter().flatten().filter(|&&c| c > 1).count()
}

fn part_2(input: &Input) -> usize {
    let mut map = Map::with_size(input.width(), input.height());

    for line in input.lines.iter() {
        for line_point in line.points_iter() {
            map.values[line_point.y][line_point.x] += 1;
        }
    }

    map.values.iter().flatten().filter(|&&c| c > 1).count()
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    let input = Input::from_str(&input);

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
