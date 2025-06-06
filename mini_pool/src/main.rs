use macroquad::prelude::*;

mod game_state;
use game_state::*;

#[macroquad::main("Mini Pool Game")]
async fn main() {
    let mut game_state = GameState::new().await;

    loop {
        if is_key_pressed(KeyCode::Space) {
            game_state.shoot_q_ball();
        }

        let dt = get_frame_time(); 
        game_state.update(dt);

        game_state.draw(); 

        next_frame().await
    }
}