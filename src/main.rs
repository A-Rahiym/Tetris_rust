use macroquad::prelude::*;


#[macroquad::main("Tetris")]
async fn main() {
    loop {
        clear_background(RED);
        draw_line(40.0, 0.0, 40.0, 40.0, 15.0, BLUE);
        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }

   
}
