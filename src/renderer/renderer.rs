use macroquad::prelude::*;
use crate::board::cell::Cell;

use crate::game::game::Game;

pub struct Renderer;

impl Renderer {
    pub fn draw(game: &Game) {
        Self::draw_board(game);

        Self::draw_active_piece(game);

        Self::draw_next_piece(game);

        Self::draw_score(game);

        Self::draw_level(game);
    }

    fn draw_board(game: &Game) {

        // Iterate over every board cell

        // Draw each occupied cell
    }

    fn draw_active_piece(game: &Game) {

        // Draw the currently falling piece
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
