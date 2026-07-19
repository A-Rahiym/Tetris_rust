
use crate::board::cell::Cell;

#[derive(Clone, Debug, PartialEq)]
pub struct Board{
    pub cells: [[Cell; 10]; 20],
}