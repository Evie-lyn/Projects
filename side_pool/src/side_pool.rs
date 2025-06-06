use macroquad::prelude::*;

// Constants
pub const TABLE_Y_POS: f32 = 300.0; // Brown part of table
pub const TABLE_THICKNESS: f32 = 50.0; // Green part of table

pub const BALL_RADIUS: f32 = 10.0; 
pub const BALL_MASS: f32 = 0.17; // in kgs, about 6 oz pool bal

pub const CUE_BALL_INITIAL_VELOCITY_MAGNITUDE: f32 = 150.0; 

// Physics Constants
pub const KINETIC_FRICTION_COEFF: f32 = 2.0; // Makes spin faster or slower
pub const ROLLING_FRICTION_COEFF: f32 = 0.005; 
pub const WALL_RESTITUTION_COEFF: f32 = 0.8; 

// Stopping
const MIN_VELOCITY_THRESHOLD: f32 = 1.0;
const MIN_ANGULAR_VELOCITY_THRESHOLD: f32 = 0.05; 
const SLIPPING_THRESHOLD: f32 = 1.0; 

pub const MOMENT_OF_INERTIA: f32 = 0.4 * BALL_MASS * BALL_RADIUS * BALL_RADIUS;

// Playable table width for each world view.
pub const WORLD_TABLE_PLAYABLE_WIDTH_FACTOR: f32 = 0.9;

pub enum GamePhase {
    Initial,    // Game just started or reset
    SetupShot,  // Spacebar pressed once
    Running,    // Spacebar preessed twice
}

pub struct SidePocket {
    pub x_position: f32, 
    pub width: f32,
    pub depth: f32,
}

impl SidePocket {
    pub fn contains_ball(&self, ball_x: f32, ball_radius: f32) -> bool {
        (ball_x - self.x_position).abs() < (self.width / 2.0 + ball_radius * 0.2)
    }
}

pub struct Ball {
    pub position: Vec2,
    pub linear_velocity: Vec2,
    pub angular_velocity: f32, 
    pub angle: f32,            
    pub color: Color,
    pub is_q_ball: bool,
    pub active: bool, 
    pub is_slipping: bool, 
    pub has_angular_momentum: bool, 
}

impl Ball {
    pub fn new(position: Vec2, color: Color, is_q_ball: bool, has_angular_momentum: bool) -> Self {
        Self {
            position,
            linear_velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            angle: 0.0,
            color,
            is_q_ball,
            active: true,
            is_slipping: false,
            has_angular_momentum,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.active { return; }

        let normal_force_magnitude = BALL_MASS * 9.81; 

        // Determine if ball is stopped before applying friction
        if self.linear_velocity.length() < MIN_VELOCITY_THRESHOLD && self.angular_velocity.abs() < MIN_ANGULAR_VELOCITY_THRESHOLD {
            self.linear_velocity = Vec2::ZERO;
            self.angular_velocity = 0.0;
            self.is_slipping = false;
            return; // Ball is stopped
        }

        let friction_force_x: f32;
        let friction_torque: f32;

        // Velocity at the bottom of the ball
        let relative_velocity_at_contact_x = self.linear_velocity.x - self.angular_velocity * BALL_RADIUS;

        if self.has_angular_momentum {
            if relative_velocity_at_contact_x.abs() > SLIPPING_THRESHOLD {
                self.is_slipping = true;
                let sign = relative_velocity_at_contact_x.signum(); 
                
                friction_force_x = -sign * KINETIC_FRICTION_COEFF * normal_force_magnitude;
                
                friction_torque = sign * KINETIC_FRICTION_COEFF * normal_force_magnitude * BALL_RADIUS; 
            } else {
                self.is_slipping = false;
                friction_force_x = -self.linear_velocity.x * ROLLING_FRICTION_COEFF;
                friction_torque = -self.angular_velocity * ROLLING_FRICTION_COEFF * BALL_RADIUS;

                let target_angular_velocity = self.linear_velocity.x / BALL_RADIUS;
                let angular_correction_rate = 0.1; 
                self.angular_velocity = lerp(self.angular_velocity, target_angular_velocity, angular_correction_rate);
            }
        } else {
            self.is_slipping = true;
            let sign = self.linear_velocity.x.signum();
            friction_force_x = -sign * KINETIC_FRICTION_COEFF * normal_force_magnitude;
            friction_torque = 0.0; // No angular effects
        }

        // Apply forces and torques
        self.linear_velocity.x += (friction_force_x / BALL_MASS) * dt;
        if self.has_angular_momentum {
            self.angular_velocity += (friction_torque / MOMENT_OF_INERTIA) * dt;
        }

        self.position.x += self.linear_velocity.x * dt; // Only horizontal movement
        if self.has_angular_momentum {
            self.angle += self.angular_velocity * dt; 
            self.angle %= std::f32::consts::TAU; // Keep angle within 0 to 2π
        } else {
            self.angle = 0.0; // No rotation if no angular momentum
        }
    }

    pub fn draw(&self, offset_x: f32) {
        if !self.active { return; }

        let draw_pos = Vec2::new(self.position.x + offset_x, self.position.y);

        // Create the ball
        draw_circle(draw_pos.x, draw_pos.y, BALL_RADIUS, self.color);

        // Make rotation indicator dots
        if self.is_q_ball && self.has_angular_momentum {
            let num_dots = 4;
            for i in 0..num_dots {
                let dot_angle = self.angle + (std::f32::consts::TAU / num_dots as f32) * i as f32;
                let dot_offset_x = dot_angle.cos() * BALL_RADIUS * 0.7; 
                let dot_offset_y = dot_angle.sin() * BALL_RADIUS * 0.7;
                draw_circle(draw_pos.x + dot_offset_x, draw_pos.y + dot_offset_y, BALL_RADIUS * 0.15, BLACK);
            }
        }

        // Draw slipping indicator if it's slipping
        if self.is_slipping {
            draw_circle(draw_pos.x, draw_pos.y, BALL_RADIUS * 0.4, RED);
        }
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub struct SidePoolWorld {
    pub balls: Vec<Ball>,
    pub screen_x_offset: f32,
    pub game_phase: GamePhase,
    initial_q_ball_pos: Vec2,
    initial_colored_ball_pos: Vec2,
    pub playable_table_width: f32, // Width of the pool table
    pub screen_view_width: f32,    // The total width on the screen
    pockets: Vec<SidePocket>,      // Pockets for pool table
}

impl SidePoolWorld {
    pub fn new(screen_x_offset: f32, has_angular_momentum: bool, screen_view_width: f32) -> Self {
        let playable_table_width = screen_view_width * WORLD_TABLE_PLAYABLE_WIDTH_FACTOR;

        let q_ball_y = TABLE_Y_POS - BALL_RADIUS;
        let colored_ball_y = TABLE_Y_POS - BALL_RADIUS;

        let q_ball_x = playable_table_width * 0.1; 
        let colored_ball_x = playable_table_width * 0.9; 

        let mut balls = Vec::new();
        balls.push(Ball::new(Vec2::new(q_ball_x, q_ball_y), WHITE, true, has_angular_momentum));
        //Purple ball
        balls.push(Ball::new(Vec2::new(colored_ball_x, colored_ball_y), PURPLE, false, has_angular_momentum)); // Ensure purple ball is added

        let pocket_width = BALL_RADIUS * 2.5; // Pockets wider than balls
        let pocket_depth = BALL_RADIUS * 1.5; 

        // Pockets are placed at the very edges of the playable table area
        let pockets = vec![
            SidePocket { x_position: 0.0, width: pocket_width, depth: pocket_depth }, 
            SidePocket { x_position: playable_table_width, width: pocket_width, depth: pocket_depth }, 
        ];

        Self {
            balls,
            screen_x_offset,
            game_phase: GamePhase::Initial,
            initial_q_ball_pos: Vec2::new(q_ball_x, q_ball_y),
            initial_colored_ball_pos: Vec2::new(colored_ball_x, colored_ball_y),
            playable_table_width,
            screen_view_width,
            pockets,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if matches!(self.game_phase, GamePhase::Running) {
            for ball in &mut self.balls {
                ball.update(dt);
            }
            self.handle_wall_collisions(); // horizontal wall collisions 
            self.handle_ball_collisions();
            self.handle_pocketing(); 
        }
    }

    fn handle_wall_collisions(&mut self) {
        let left_table_edge = 0.0;
        let right_table_edge = self.playable_table_width;

        for ball in &mut self.balls {
            if !ball.active { continue; }

            if ball.position.x - BALL_RADIUS < left_table_edge && !self.pockets[0].contains_ball(ball.position.x, BALL_RADIUS) {
                ball.position.x = left_table_edge + BALL_RADIUS;
                ball.linear_velocity.x *= -WALL_RESTITUTION_COEFF; // Bounce
                ball.linear_velocity.x *= (0.5_f32).sqrt(); // Collision
                if ball.has_angular_momentum {
                    ball.angular_velocity *= -WALL_RESTITUTION_COEFF; 
                }
            }
            else if ball.position.x + BALL_RADIUS > right_table_edge && !self.pockets[1].contains_ball(ball.position.x, BALL_RADIUS) {
                ball.position.x = right_table_edge - BALL_RADIUS;
                ball.linear_velocity.x *= -WALL_RESTITUTION_COEFF; // Bounce
                ball.linear_velocity.x *= (0.5_f32).sqrt(); // Collision
                if ball.has_angular_momentum {
                    ball.angular_velocity *= -WALL_RESTITUTION_COEFF; 
                }
            }
        }
    }

    fn handle_ball_collisions(&mut self) {
        if self.balls.len() < 2 { return; }

        let q_ball_idx = self.balls.iter().position(|b| b.is_q_ball).unwrap();
        let other_ball_idx = self.balls.iter().position(|b| !b.is_q_ball).unwrap();

        let (ball1, ball2) = {
            let (mut_ref1, mut_ref2);
            if q_ball_idx < other_ball_idx {
                let (left_part, right_part) = self.balls.split_at_mut(other_ball_idx);
                mut_ref1 = &mut left_part[q_ball_idx];
                mut_ref2 = &mut right_part[0];
            } else {
                let (left_part, right_part) = self.balls.split_at_mut(q_ball_idx);
                mut_ref1 = &mut left_part[other_ball_idx];
                mut_ref2 = &mut right_part[0];
            }
            (mut_ref1, mut_ref2)
        };

        let distance = (ball1.position.x - ball2.position.x).abs();
        let combined_radii = BALL_RADIUS * 2.0;

        let relative_velocity_x = ball1.linear_velocity.x - ball2.linear_velocity.x;
        let is_approaching = (ball2.position.x - ball1.position.x).signum() == relative_velocity_x.signum();

        if distance < combined_radii && is_approaching {
            let overlap = combined_radii - distance;
            let sign_dir = (ball1.position.x - ball2.position.x).signum();
            ball1.position.x += sign_dir * (overlap / 2.0);
            ball2.position.x -= sign_dir * (overlap / 2.0);

            // Collision
            let transfer_factor = 0.9; // How much of velocity the q ball gives to purple ball
            let cue_ball_retained_velocity = ball1.linear_velocity.x * (1.0 - transfer_factor);
            let blue_ball_gains_velocity = ball1.linear_velocity.x * transfer_factor;

            ball1.linear_velocity.x = cue_ball_retained_velocity;
            ball2.linear_velocity.x = blue_ball_gains_velocity;

            // Transfer angular momentum during collision
            if ball1.has_angular_momentum && ball2.has_angular_momentum {
                let avg_angular_vel = (ball1.angular_velocity + ball2.angular_velocity) / 2.0;
                ball1.angular_velocity = avg_angular_vel * 0.5; 
                ball2.angular_velocity = avg_angular_vel * 0.5;
            } else if ball1.has_angular_momentum {
                ball1.angular_velocity *= 0.5; 
            }
        }
    }

    fn handle_pocketing(&mut self) {
        for pocket in &self.pockets {
            for ball in &mut self.balls {
                if ball.active && pocket.contains_ball(ball.position.x, BALL_RADIUS) {
                    ball.active = false; // Mark ball as potted
                    ball.linear_velocity = Vec2::ZERO; // Stop poyted balls movement
                    ball.angular_velocity = 0.0; // Stop  spin
                    ball.position.y = TABLE_Y_POS + TABLE_THICKNESS + pocket.depth + BALL_RADIUS * 5.0;
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.game_phase = GamePhase::Initial;
        // Reset Q-ball
        if let Some(q_ball) = self.balls.iter_mut().find(|b| b.is_q_ball) {
            q_ball.position = self.initial_q_ball_pos;
            q_ball.linear_velocity = Vec2::ZERO;
            q_ball.angular_velocity = 0.0;
            q_ball.angle = 0.0;
            q_ball.is_slipping = false;
            q_ball.active = true;
        }

        // Reset purple balls
        if let Some(other_ball) = self.balls.iter_mut().find(|b| !b.is_q_ball) {
            other_ball.position = self.initial_colored_ball_pos;
            other_ball.linear_velocity = Vec2::ZERO;
            other_ball.angular_velocity = 0.0;
            other_ball.angle = 0.0;
            other_ball.is_slipping = false;
            other_ball.active = true;
        }
    }

    pub fn shoot_q_ball(&mut self) {
        if let Some(q_ball) = self.balls.iter_mut().find(|b| b.is_q_ball) {
            q_ball.linear_velocity.x = CUE_BALL_INITIAL_VELOCITY_MAGNITUDE;

            if q_ball.has_angular_momentum {
                // Initial angular velocity to 0.0 to gain spin
                q_ball.angular_velocity = 0.0; 
            }
            self.game_phase = GamePhase::Running;
        }
    }

    pub fn draw(&self) {
        let table_draw_offset_x = self.screen_x_offset + (self.screen_view_width - self.playable_table_width) / 2.0;

        // Draw the pool table as a rectangle
        draw_rectangle(
            table_draw_offset_x,
            TABLE_Y_POS,
            self.playable_table_width,
            TABLE_THICKNESS,
            DARKGREEN, // Green
        );

        // Draw brown part
        draw_line(
            table_draw_offset_x, TABLE_Y_POS,
            table_draw_offset_x + self.playable_table_width, TABLE_Y_POS,
            4.0, BROWN // brown
        );

        draw_rectangle(self.screen_x_offset, TABLE_Y_POS, (self.screen_view_width - self.playable_table_width) / 2.0, TABLE_THICKNESS, BROWN); // Left rail
        draw_rectangle(self.screen_x_offset + self.playable_table_width + (self.screen_view_width - self.playable_table_width) / 2.0, TABLE_Y_POS, (self.screen_view_width - self.playable_table_width) / 2.0, TABLE_THICKNESS, BROWN); // Right rail

        // Draw pockets
        for pocket in &self.pockets {
            draw_rectangle(
                table_draw_offset_x + pocket.x_position - pocket.width / 2.0,
                TABLE_Y_POS,
                pocket.width,
                pocket.depth + TABLE_THICKNESS, 
                BLACK, // Black
            );
            draw_line(
                table_draw_offset_x + pocket.x_position - pocket.width / 2.0, TABLE_Y_POS + 1.0,
                table_draw_offset_x + pocket.x_position + pocket.width / 2.0, TABLE_Y_POS + 1.0,
                2.0, BROWN
            );
        }

        // Draw balls
        for ball in &self.balls {
            ball.draw(table_draw_offset_x); 
        }

        if let Some(q_ball) = self.balls.iter().find(|b| b.is_q_ball) {
            draw_text(
                &format!("Linear V: {:.2} px/s", q_ball.linear_velocity.x),
                self.screen_x_offset + 10.0, 
                20.0,
                20.0,
                BLACK,
            );
            if q_ball.has_angular_momentum {
                draw_text(
                    &format!("Angular V: {:.2} rad/s", q_ball.angular_velocity),
                    self.screen_x_offset + 10.0,
                    40.0,
                    20.0,
                    BLACK,
                );
                draw_text(
                    &format!("Slipping: {}", q_ball.is_slipping),
                    self.screen_x_offset + 10.0,
                    60.0,
                    20.0,
                    BLACK,
                );
            }
        }
    }
}


pub struct SidePoolGame {
    world_with_am: SidePoolWorld,
    world_without_am: SidePoolWorld,
    game_phase_global: GamePhase, 
}

impl SidePoolGame {
    pub async fn new() -> Self {
        let full_screen_width = screen_width();
        let screen_width_for_each_world_view = full_screen_width / 2.0; //Split screen
        let world_with_am = SidePoolWorld::new(0.0, true, screen_width_for_each_world_view); // Left half
        let world_without_am = SidePoolWorld::new(screen_width_for_each_world_view, false, screen_width_for_each_world_view); // Right half

        Self {
            world_with_am,
            world_without_am,
            game_phase_global: GamePhase::Initial,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::Space) {
            // Spacebar resets
            if matches!(self.game_phase_global, GamePhase::Initial | GamePhase::Running) {
                self.world_with_am.reset();
                self.world_without_am.reset();
                self.game_phase_global = GamePhase::SetupShot;
            } else if matches!(self.game_phase_global, GamePhase::SetupShot) {
                // Spacebar shoots
                self.world_with_am.shoot_q_ball();
                self.world_without_am.shoot_q_ball();
                self.game_phase_global = GamePhase::Running;
            }
            return; // Exit after press spacebar
        }

        if matches!(self.game_phase_global, GamePhase::Running) {
            self.world_with_am.update(dt);
            self.world_without_am.update(dt);

            let all_stopped_am = self.world_with_am.balls.iter()
                .filter(|b| b.active) // Only consider active balls
                .all(|b| b.linear_velocity.length() < MIN_VELOCITY_THRESHOLD && b.angular_velocity.abs() < MIN_ANGULAR_VELOCITY_THRESHOLD);

            let all_stopped_no_am = self.world_without_am.balls.iter()
                .filter(|b| b.active) // Only consider active balls
                .all(|b| b.linear_velocity.length() < MIN_VELOCITY_THRESHOLD);

            if all_stopped_am && all_stopped_no_am {
                self.game_phase_global = GamePhase::Initial;
            }
        }
    }

    pub fn draw(&self) {
        clear_background(LIGHTGRAY);

        // Draw the split screen divider
        draw_line(screen_width() / 2.0, 0.0, screen_width() / 2.0, screen_height(), 2.0, BLACK);
        draw_text("With Angular Momentum", screen_width() / 4.0 - 80.0, 90.0, 20.0, BLACK);
        draw_text("Without Angular Momentum", screen_width() * 3.0 / 4.0 - 100.0, 90.0, 20.0, BLACK);


        self.world_with_am.draw();
        self.world_without_am.draw();
    }
}