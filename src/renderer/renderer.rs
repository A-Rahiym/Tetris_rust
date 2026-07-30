use crate::board::cell::Cell;
use crate::game::game::Game;
use crate::piece::kind::PieceKind;
use macroquad::prelude::*;

pub struct Renderer;

const CELL_SIZE: f32 = 32.0;
const BOARD_X: f32 = 100.0;
const BOARD_Y: f32 = 50.0;

impl Renderer {
    pub fn draw(game: &Game) {
        Self::draw_board(game);

        Self::draw_active_piece(game);

        Self::draw_next_piece(game);

        Self::draw_score(game);

        Self::draw_level(game);
    }

    fn piece_color(kind: PieceKind) -> Color {
        match kind {
            PieceKind::I => SKYBLUE,
            PieceKind::O => YELLOW,
            PieceKind::T => PURPLE,
            PieceKind::S => GREEN,
            PieceKind::Z => RED,
            PieceKind::L => ORANGE,
            PieceKind::J => BLUE,
        }
    }

    fn draw_board(game: &Game) {
        for (row, cells) in game.board.cells.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                let x = BOARD_X + col as f32 * CELL_SIZE;
                let y = BOARD_Y + row as f32 * CELL_SIZE;

                match cell {
                    Cell::Empty => {
                        draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, DARKGRAY);
                    }
                    Cell::Filled(kind) => {
                        draw_rectangle(
                            x,
                            y,
                            CELL_SIZE,
                            CELL_SIZE,
                            Self::piece_color(*kind),
                        );
                    }
                }
            }
        }
    }

    fn draw_active_piece(game: &Game) {
        let piece = &game.active_pieces;

        for row in 0..4 {
            for col in 0..4 {
                if piece.shape.cells[row][col] {
                    draw_rectangle(
                        BOARD_X + ((piece.position.x + col as i32) as f32 * CELL_SIZE),
                        BOARD_Y + ((piece.position.y + row as i32) as f32 * CELL_SIZE),
                        CELL_SIZE,
                        CELL_SIZE,
                        Self::piece_color(piece.kind),
                    );
                }
            }
        }
    }

    fn draw_next_piece(game: &Game) {

        // Draw the next falling piece
    }

    fn draw_score(game: &Game) {

        // Draw score
    }

    fn draw_level(game: &Game) {
        // Draw Level
    }
}