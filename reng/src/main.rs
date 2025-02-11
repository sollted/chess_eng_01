use macroquad::prelude::*;
mod board;
mod window;

use window::GameWindow;

#[macroquad::main("Chess")]
async fn main() {
    let mut game_window = GameWindow::new().await;

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));
        
        game_window.update();
        game_window.draw();

        next_frame().await;
    }
}
