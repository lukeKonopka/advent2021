#[derive(Clone, Copy, Debug)]
pub enum BoardValue {
    Unmarked(u8),
    Marked(u8),
}

impl BoardValue {
    pub fn get_number(&self) -> u8 {
        match self {
            BoardValue::Unmarked(n) => *n,
            BoardValue::Marked(n) => *n,
        }
    }

    pub fn is_marked(&self) -> bool {
        match self {
            BoardValue::Unmarked(_) => false,
            BoardValue::Marked(_) => true,
        }
    }

    pub fn mark(&mut self) {
        *self = BoardValue::Marked(self.get_number())
    }
}

#[derive(Debug, Clone)]
pub struct BingoBoard {
    values: [[BoardValue; 5]; 5],
}

impl BingoBoard {
    pub fn mark_number(&mut self, number: u8) {
        for rows in self.values.iter_mut() {
            for value in rows.iter_mut() {
                if value.get_number() == number {
                    value.mark();
                }
            }
        }
    }

    fn transpose(&self) -> Self {
        let mut new_values = self.values.clone();
        let n = self.values.len();

        for row_idx in 0..n {
            for col_idx in 0..n {
                let value = self.values[row_idx][col_idx];
                new_values[col_idx][row_idx] = value;
            }
        }

        BingoBoard { values: new_values }
    }

    pub fn is_winning(&self) -> bool {
        let row_completed = self
            .values
            .iter()
            .any(|row| row.iter().all(BoardValue::is_marked));

        let col_completed = self
            .transpose()
            .values
            .iter()
            .any(|col| col.iter().all(BoardValue::is_marked));

        row_completed || col_completed
    }

    pub fn unmarked_iter(&self) -> impl Iterator<Item = &BoardValue> {
        self.values
            .iter()
            .map(|row| row.iter())
            .flatten()
            .filter(|v| !v.is_marked())
    }

    pub fn from_str(source: &str) -> Self {
        let values_vec = source
            .lines()
            .map(|row_str| {
                row_str
                    .split_whitespace()
                    .map(|value_str| value_str.parse::<u8>().expect("cannot parse value as u8"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut values = [[BoardValue::Unmarked(0); 5]; 5];
        for row_idx in 0..5 {
            for col_idx in 0..5 {
                values[row_idx][col_idx] = BoardValue::Unmarked(values_vec[row_idx][col_idx]);
            }
        }

        Self { values }
    }
}
