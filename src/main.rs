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

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: 900,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BLACK);

        let action = input::input::poll();
        game.update(action);

        Renderer::draw(&game);

        if game.state == game::state::GameState::GameOver {
            draw_text("GAME OVER", 300.0, 360.0, 50.0, RED);
        }

        next_frame().await
    }
}
