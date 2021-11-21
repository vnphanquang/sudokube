use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate(pub u8, pub u8);

impl Coordinate {
    pub fn row(&self) -> u8 {
        self.0
    }

    pub fn col(&self) -> u8 {
        self.1
    }

    pub fn shift(&self, rows: Option<u8>, cols: Option<u8>) -> Coordinate {
        Coordinate(
            self.row() + rows.unwrap_or(0),
            self.col() + cols.unwrap_or(0),
        )
    }
}

impl std::ops::Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Self(row1, col1) = self;
        let Self(row2, col2) = other;
        Self(row1 + row2, col1 + col2)
    }
}
