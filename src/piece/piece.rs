use crate::piece::kind::PieceKind;
use crate::piece::position::Position;
use crate::piece::rotation::Rotation;

#[ derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub kind: PieceKind,
    pub position: Position,
    pub rotation: Rotation,
}