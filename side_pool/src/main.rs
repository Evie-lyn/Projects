use macroquad::prelude::*;
mod side_pool; 
use side_pool::*; 

#[macroquad::main("Side Pool Game")]
async fn main() {
    let mut game = SidePoolGame::new().await; 

    loop {
        let dt = get_frame_time();
        game.update(dt);
        game.draw(); 

        next_frame().await
    }
}