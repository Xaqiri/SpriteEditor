use macroquad::prelude::*;

#[macroquad::main("Sprite Editor")]
async fn main() {
    loop {
        request_new_screen_size(800.0, 600.0);
        clear_background(BLUE);
        next_frame().await
    }
}
