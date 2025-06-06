use macroquad::prelude::*;
use ::rand::{
    rngs::ThreadRng,
    Rng,
};
use ::rand::thread_rng;

pub const RECT_WIDTH: f32 = 600.0;
pub const RECT_HEIGHT: f32 = 300.0;
pub const BALL_RADIUS: f32 = 10.0;
const INITIAL_Q_VELOCITY_MAGNITUDE: f32 = 500.0;
const VELOCITY_DECAY_RATE: f32 = 0.98;
const MIN_VELOCITY_THRESHOLD: f32 = 10.0;

pub struct Pocket {
    pub position: Vec2,
    pub radius: f32,
}

impl Pocket {
    pub fn contains_ball(&self, ball_pos: Vec2) -> bool {
        ball_pos.distance(self.position) < self.radius
    }
}

pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
    pub is_q_ball: bool,
    pub active: bool,
}

impl Ball {
    pub fn new(position: Vec2, color: Color, is_q_ball: bool) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            color,
            is_q_ball,
            active: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.active { return; }

        self.position += self.velocity * dt;

        if self.velocity.length() > MIN_VELOCITY_THRESHOLD {
            self.velocity *= (VELOCITY_DECAY_RATE).powf(dt);
        } else {
            self.velocity = Vec2::ZERO;
        }
    }

    pub fn draw(&self) {
        if self.active {
            draw_circle(self.position.x, self.position.y, BALL_RADIUS, self.color);
        }
    }
}

pub struct GameState {
    pub balls: Vec<Ball>,
    pub rectangle_top_left: Vec2,
    pub pockets: Vec<Pocket>,
    rand: ThreadRng,
}

impl GameState {
    pub async fn new() -> GameState {
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let rectangle_top_left = Vec2::new(
            screen_center.x - RECT_WIDTH / 2.0,
            screen_center.y - RECT_HEIGHT / 2.0,
        );

        let mut balls = Vec::new();

        // --- Q-Ball Position ---
        // Place the Q-ball at the top end of the table, slightly below the center-line
        // (This simulates being behind the "head string" line)
        let q_ball_position = Vec2::new(
            screen_center.x,
            screen_center.y - RECT_HEIGHT / 4.0, // Adjust this value as needed
        );
        balls.push(Ball::new(q_ball_position, WHITE, true));

        // --- Racked Balls Position (Apex facing Q-Ball) ---
        // These balls will be placed at the opposite end of the table,
        // with the top ball of the triangle (the apex) pointing towards the Q-ball.
        let rack_apex_y = screen_center.y + RECT_HEIGHT / 4.0; // This is the Y-coordinate for the front ball of the rack
        let rack_center_x = screen_center.x;

        // Apex ball (front of the triangle)
        balls.push(Ball::new(Vec2::new(rack_center_x, rack_apex_y), BLUE, false));

        // Second row of the triangle (two balls behind the apex)
        let row2_y = rack_apex_y + BALL_RADIUS * 2.0 * 0.866; // approx BALL_RADIUS * sqrt(3)
        let row2_x_offset = BALL_RADIUS;
        balls.push(Ball::new(Vec2::new(rack_center_x - row2_x_offset, row2_y), RED, false));
        balls.push(Ball::new(Vec2::new(rack_center_x + row2_x_offset, row2_y), YELLOW, false));

        // --- Pocket Definitions ---
        let pocket_radius = BALL_RADIUS * 1.5;
        let rect_right = rectangle_top_left.x + RECT_WIDTH;
        let rect_bottom = rectangle_top_left.y + RECT_HEIGHT;

        // Pockets are placed at the corners and mid-points of the table edges
        let pockets = vec![
            // Top Pockets
            Pocket { position: Vec2::new(rectangle_top_left.x, rectangle_top_left.y), radius: pocket_radius }, // Top-left
            Pocket { position: Vec2::new(rectangle_top_left.x + RECT_WIDTH / 2.0, rectangle_top_left.y), radius: pocket_radius }, // Top-middle
            Pocket { position: Vec2::new(rect_right, rectangle_top_left.y), radius: pocket_radius }, // Top-right
            // Bottom Pockets
            Pocket { position: Vec2::new(rectangle_top_left.x, rect_bottom), radius: pocket_radius }, // Bottom-left
            Pocket { position: Vec2::new(rectangle_top_left.x + RECT_WIDTH / 2.0, rect_bottom), radius: pocket_radius }, // Bottom-middle
            Pocket { position: Vec2::new(rect_right, rect_bottom), radius: pocket_radius }, // Bottom-right
        ];

        GameState {
            balls,
            rectangle_top_left,
            pockets,
            rand: thread_rng(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        for ball in &mut self.balls {
            ball.update(dt);
        }

        self.handle_wall_collisions();

        self.handle_ball_collisions();

        self.handle_pocketing();
    }

    fn handle_wall_collisions(&mut self) {
        let rect_left = self.rectangle_top_left.x;
        let rect_right = self.rectangle_top_left.x + RECT_WIDTH;
        let rect_top = self.rectangle_top_left.y;
        let rect_bottom = self.rectangle_top_left.y + RECT_HEIGHT;

        for ball in &mut self.balls {
            if !ball.active { continue; }

            if ball.position.x - BALL_RADIUS < rect_left {
                ball.position.x = rect_left + BALL_RADIUS;
                ball.velocity.x *= -1.0;
            } else if ball.position.x + BALL_RADIUS > rect_right {
                ball.position.x = rect_right - BALL_RADIUS;
                ball.velocity.x *= -1.0;
            }

            if ball.position.y - BALL_RADIUS < rect_top {
                ball.position.y = rect_top + BALL_RADIUS;
                ball.velocity.y *= -1.0;
            } else if ball.position.y + BALL_RADIUS > rect_bottom {
                ball.position.y = rect_bottom - BALL_RADIUS;
                ball.velocity.y *= -1.0;
            }
        }
    }

    fn handle_ball_collisions(&mut self) {
        for i in 0..self.balls.len() {
            if !self.balls[i].active { continue; }

            for j in (i + 1)..self.balls.len() {
                if !self.balls[j].active { continue; }

                let ball1_pos = self.balls[i].position;
                let ball2_pos = self.balls[j].position;

                let distance = ball1_pos.distance(ball2_pos);
                let combined_radii = BALL_RADIUS * 2.0;

                if distance < combined_radii {

                    let (left, right) = self.balls.split_at_mut(j);
                    let ball1 = &mut left[i];
                    let ball2 = &mut right[0];

                    let overlap = combined_radii - distance;
                    let collision_normal = (ball1.position - ball2.position).normalize();
                    ball1.position += collision_normal * (overlap / 2.0);
                    ball2.position -= collision_normal * (overlap / 2.0);


                    let v1 = ball1.velocity;
                    let v2 = ball2.velocity;

                    let normal = (ball1.position - ball2.position).normalize();

                    let tangent = Vec2::new(-normal.y, normal.x);

                    let v1n = v1.dot(normal);
                    let v1t = v1.dot(tangent);
                    let v2n = v2.dot(normal);
                    let v2t = v2.dot(tangent);

                    let v1n_final = v2n;
                    let v2n_final = v1n;

                    let v1n_vec = v1n_final * normal;
                    let v1t_vec = v1t * tangent;
                    let v2n_vec = v2n_final * normal;
                    let v2t_vec = v2t * tangent;

                    ball1.velocity = v1n_vec + v1t_vec;
                    ball2.velocity = v2n_vec + v2t_vec;
                }
            }
        }
    }

    pub fn shoot_q_ball(&mut self) {
        if let Some(q_ball) = self.balls.iter_mut().find(|b| b.is_q_ball) {
            let angle: f32 = self.rand.gen_range(0.0..std::f32::consts::TAU);
            q_ball.velocity.x = INITIAL_Q_VELOCITY_MAGNITUDE * angle.cos();
            q_ball.velocity.y = INITIAL_Q_VELOCITY_MAGNITUDE * angle.sin();
        }
    }
    fn handle_pocketing(&mut self) {
        for pocket in &self.pockets {
            for ball in &mut self.balls {
                if ball.active && pocket.contains_ball(ball.position) {
                    ball.active = false;
                    ball.velocity = Vec2::ZERO;
                }
            }
        }
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);

        draw_rectangle(
            self.rectangle_top_left.x,
            self.rectangle_top_left.y,
            RECT_WIDTH,
            RECT_HEIGHT,
            DARKGREEN,
        );

        for pocket in &self.pockets {
            draw_circle(pocket.position.x, pocket.position.y, pocket.radius, BLACK);
        }

        for ball in &self.balls {
            ball.draw();
        }

        if let Some(q_ball) = self.balls.iter().find(|b| b.is_q_ball) {
            draw_text(
                &format!(
                    "Q-Ball Velocity: ({:.2}, {:.2})",
                    q_ball.velocity.x, q_ball.velocity.y
                ),
                10.0,
                20.0,
                20.0,
                BLACK,
            );
        }

        let active_balls_count = self.balls.iter().filter(|b| b.active).count();
         draw_text(
            &format!("Active Balls: {}", active_balls_count),
            10.0,
            40.0,
            20.0,
            BLACK,
        );
    }
}