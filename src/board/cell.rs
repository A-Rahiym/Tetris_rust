use crate::piece::kind::PieceKind;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]

pub enum Cell {
Empty,
Filled(PieceKind)
}