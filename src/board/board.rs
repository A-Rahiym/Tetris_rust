use crate::board::cell::Cell;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub cells: [[Cell; 10]; 20],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; 10]; 20],
        }
    }
}
