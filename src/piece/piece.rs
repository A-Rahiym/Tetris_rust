use crate::piece::kind::PieceKind;
use crate::piece::position::Position;
use crate::piece::rotation::Rotation;
use crate::piece::shape::{Shape, get_shape};

#[ derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub kind: PieceKind,
    pub position: Position,
    pub rotation: Rotation,
    pub shape: Shape,
}

impl Piece {
    pub fn new(
        kind: PieceKind,
        position: Position,
        rotation: Rotation,
    ) -> Self {
        let shape = get_shape(kind, rotation);

        Self {
            kind,
            shape,
            position,
            rotation,
        }
    }

    pub fn rotate_clockwise(&mut self) {
        self.rotation = match self.rotation {
            Rotation::North => Rotation::East,
            Rotation::East => Rotation::South,
            Rotation::South => Rotation::West,
            Rotation::West => Rotation::North,
        };
        self.shape = get_shape(self.kind, self.rotation);
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.rotation = match self.rotation {
            Rotation::North => Rotation::West,
            Rotation::West => Rotation::South,
            Rotation::South => Rotation::East,
            Rotation::East => Rotation::North,
        };
        self.shape = get_shape(self.kind, self.rotation);
    }
}