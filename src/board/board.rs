use crate::board::cell::Cell;
use crate::piece::piece::Piece;

pub const BOARD_WIDTH: usize = 20;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub cells: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
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

                if x < 0 || x >= BOARD_WIDTH as i32 || y < 0 || y >= BOARD_HEIGHT as i32 {
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