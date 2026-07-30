
use crate::board::cell::Cell;
use crate::game::game::Game;


pub struct Renderer;

#[derive(Clone, Debug, PartialEq)]
pub struct Board{
    pub cells: [[Cell; 10]; 20],
}

