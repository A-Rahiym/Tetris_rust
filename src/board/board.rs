use crate::board::cell::Cell;
use crate::piece::piece::Piece;

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

    pub fn is_valid(&self, piece: &Piece) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if !piece.shape.cells[row][col] {
                    continue;
                }
                let x = piece.position.x + col as i32;
                let y = piece.position.y + row as i32;

                if x < 0 || x >= 10 || y < 0 || y >= 20 {
                    return false;
                }
                if let Cell::Filled(_) = self.cells[y as usize][x as usize] {
                    return false;
                }
            }
        }
        true
    }
}