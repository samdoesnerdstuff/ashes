mod player;
use player::Player;

use macroquad::prelude::*;

#[macroquad::main("Ashes")]
async fn main() {
    let mut player = Player::new(0.0, 0.0);
    let world_width: f32 = 960.0;
    let world_height: f32 = 720.0;

    loop {
        clear_background(BLACK);
        let dt: f32 = get_frame_time();

        player.update(dt, world_width, world_height);
        player.draw();

        next_frame().await
    }
    
}
