mod board;
mod game;
mod piece;
mod score;
mod timing;
mod input;
mod renderer;

use macroquad::prelude::*;

use crate::game::game::Game;
use crate::renderer::renderer::Renderer;

#[macroquad::main("Tetris")]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BLACK);

        Renderer::draw(&game);

        next_frame().await
    }
}
