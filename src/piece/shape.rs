use crate::piece::kind::PieceKind;
use crate::piece::rotation::Rotation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shape {
    pub cells: [[bool; 4]; 4],
}

pub fn get_shape(kind: PieceKind, rotation: Rotation) -> Shape {
    match (kind, rotation) {
        // =========================
        // I
        // =========================

        (PieceKind::I, Rotation::North) => Shape {
            cells: [
                [false, false, false, false],
                [true,  true,  true,  true ],
                [false, false, false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::I, Rotation::East) => Shape {
            cells: [
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
            ],
        },

        (PieceKind::I, Rotation::South) => Shape {
            cells: [
                [false, false, false, false],
                [false, false, false, false],
                [true,  true,  true,  true ],
                [false, false, false, false],
            ],
        },

        (PieceKind::I, Rotation::West) => Shape {
            cells: [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
        },

        // =========================
        // O
        // =========================

        (PieceKind::O, _) => Shape {
            cells: [
                [false, true,  true,  false],
                [false, true,  true,  false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        },

        // =========================
        // T
        // =========================

        (PieceKind::T, Rotation::North) => Shape {
            cells: [
                [false, true, false, false],
                [true,  true, true,  false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::T, Rotation::East) => Shape {
            cells: [
                [false, true, false, false],
                [false, true, true,  false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::T, Rotation::South) => Shape {
            cells: [
                [false, false, false, false],
                [true,  true,  true,  false],
                [false, true,  false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::T, Rotation::West) => Shape {
            cells: [
                [false, true, false, false],
                [true,  true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        },

        // =========================
        // S
        // =========================

        (PieceKind::S, Rotation::North)
        | (PieceKind::S, Rotation::South) => Shape {
            cells: [
                [false, true,  true,  false],
                [true,  true,  false, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::S, Rotation::East)
        | (PieceKind::S, Rotation::West) => Shape {
            cells: [
                [false, true,  false, false],
                [false, true,  true,  false],
                [false, false, true,  false],
                [false, false, false, false],
            ],
        },

        // =========================
        // Z
        // =========================

        (PieceKind::Z, Rotation::North)
        | (PieceKind::Z, Rotation::South) => Shape {
            cells: [
                [true,  true,  false, false],
                [false, true,  true,  false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::Z, Rotation::East)
        | (PieceKind::Z, Rotation::West) => Shape {
            cells: [
                [false, false, true,  false],
                [false, true,  true,  false],
                [false, true,  false, false],
                [false, false, false, false],
            ],
        },

        // =========================
        // J
        // =========================

        (PieceKind::J, Rotation::North) => Shape {
            cells: [
                [true,  false, false, false],
                [true,  true,  true,  false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::J, Rotation::East) => Shape {
            cells: [
                [false, true, true, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::J, Rotation::South) => Shape {
            cells: [
                [false, false, false, false],
                [true,  true,  true,  false],
                [false, false, true,  false],
                [false, false, false, false],
            ],
        },

        (PieceKind::J, Rotation::West) => Shape {
            cells: [
                [false, true, false, false],
                [false, true, false, false],
                [true,  true, false, false],
                [false, false, false, false],
            ],
        },

        // =========================
        // L
        // =========================

        (PieceKind::L, Rotation::North) => Shape {
            cells: [
                [false, false, true,  false],
                [true,  true,  true,  false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::L, Rotation::East) => Shape {
            cells: [
                [false, true,  false, false],
                [false, true,  false, false],
                [false, true,  true,  false],
                [false, false, false, false],
            ],
        },

        (PieceKind::L, Rotation::South) => Shape {
            cells: [
                [false, false, false, false],
                [true,  true,  true,  false],
                [true,  false, false, false],
                [false, false, false, false],
            ],
        },

        (PieceKind::L, Rotation::West) => Shape {
            cells: [
                [true,  true,  false, false],
                [false, true,  false, false],
                [false, true,  false, false],
                [false, false, false, false],
            ],
        },
    }
}