use std::ops::{Mul, AddAssign};

use winit::event::*;
use rand::Rng;

pub const SCREEN_SIZE: Vec2i = Vec2i {x: 800, y:500};

pub const PADDLE_SIZE: Vec2 = Vec2 {x: 50.0, y: 200.0};
pub const BALL_SIZE: Vec2 = Vec2 {x: 50.0, y: 50.0};

const TICKS_PER_SECOND: f32 = 60.0;
const TICK_TIME: f32 = 1.0 / TICKS_PER_SECOND;

pub const PADDLE_SPEED: f32 = 6.0;
pub const BALL_SPEED: f32 = 15.0;

pub struct GameState {
    pub player: Entity,
    pub com: Entity,
    pub ball: Entity,
    pub score: u8,
    previous_time: instant::Instant,
    tick: f32
}

pub struct Entity {
    pub quad: Quad,
    dir: Vec2
}

impl Entity {
    pub fn add_position(&mut self, pos: Vec2) {
        self.quad.pos += pos;
    }
}

pub struct Quad {
    pub pos: Vec2,
    pub size: Vec2
}

impl Quad {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self {
            pos,
            size
        }
    }

    pub fn interects(&self, b: &Quad) -> bool {
        let dx = (self.pos.x - b.pos.x).abs();
        let dy = (self.pos.y - b.pos.y).abs();

        let half_width = self.size.x / 2.0 + b.size.x / 2.0;
        let half_height = self.size.y / 2.0 + b.size.y / 2.0;

        dx < half_width && dy < half_height
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32
}

impl Vec2i {
    pub fn new<T: Into<i32>>(x: T, y: T) -> Self {
        Vec2i {
            x: x.into(),
            y: y.into()
        }
    }

    pub fn zero() -> Self {
        Vec2i::new(0,0)
    }
}

impl Mul<f32> for Vec2i {
    type Output = Vec2;

    fn mul(self, mul: f32) -> Vec2 {
        Vec2::new(self.x as f32 * mul, self.y as f32 * mul)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, mul: f32) -> Vec2 {
        Vec2::new(self.x as f32 * mul, self.y as f32 * mul)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new<T: num::ToPrimitive>(x: T, y: T) -> Self {
        Vec2 {
            x: x.to_f32().unwrap(),
            y: y.to_f32().unwrap()
        }
    }

    pub fn zero() -> Self {
        Vec2::new(0,0)
    }

    pub fn normalize(&mut self) -> Self {
        let mag = (self.x * self.x + self.y * self.y).sqrt();
        if mag != 0.0 { 
            self.x /= mag;
            self.y /= mag;
        }
        *self
    }
}

impl GameState {
    pub fn new() -> Self {

        let player = Entity {
            quad: Quad::new(Vec2::new(-700, 0), PADDLE_SIZE),
            dir: Vec2::zero()
        };

        let com = Entity {
            quad: Quad::new(Vec2::new(700, 0), PADDLE_SIZE),
            dir: Vec2::zero()
        };
        
        let ball = Entity {
            quad: Quad::new(Vec2::new(0, 0), BALL_SIZE),
            dir: Vec2::new(rand::thread_rng().gen_range(-1.0..=1.0), rand::thread_rng().gen_range(-1.0..=1.0)).normalize()
        };

        GameState {
            player,
            com,
            ball,
            previous_time: instant::Instant::now(),
            tick: 0.0,
            score: 0
        }
    }

    pub fn update(&mut self) {
        let current_time = instant::Instant::now();
        let elapsed_time = current_time.duration_since(self.previous_time).as_secs_f32();
        self.previous_time = current_time;

        self.tick += elapsed_time;

        if self.tick > TICK_TIME {
            self.player.add_position(self.player.dir * PADDLE_SPEED);
            
            if self.ball.quad.pos.y > SCREEN_SIZE.y as f32 - self.ball.quad.size.y 
            || self.ball.quad.pos.y < -SCREEN_SIZE.y as f32 + self.ball.quad.size.y {
                self.ball.dir = Vec2::new(self.ball.dir.x, -self.ball.dir.y);
            }
            if self.player.quad.interects(&self.ball.quad) || self.com.quad.interects(&self.ball.quad) {
                self.ball.dir = Vec2::new(-self.ball.dir.x, self.ball.dir.y);
            }
            self.ball.add_position(self.ball.dir * BALL_SPEED);

            if self.ball.quad.pos.x > SCREEN_SIZE.x as f32 + self.ball.quad.size.x {
                self.score += 1;
                self.ball.quad.pos = Vec2::zero();
            }
            
            if self.ball.quad.pos.x < -SCREEN_SIZE.x as f32 + self.ball.quad.size.x {
                self.ball.quad.pos = Vec2::zero();
            }

            if self.ball.dir.x > 0.0 {
                if self.ball.quad.pos.y > self.com.quad.pos.y {
                    self.com.dir = Vec2::new(0, 1);
                } else if self.ball.quad.pos.y < self.com.quad.pos.y {
                    self.com.dir = Vec2::new(0, -1);
                } else {
                    self.com.dir = Vec2::zero();
                }
            } else {
                self.com.dir = Vec2::zero();
            }

            self.com.add_position(self.com.dir * PADDLE_SPEED);

            self.tick -= TICK_TIME;
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        ..
                    },
                ..
            } => {
                self.player.dir = Vec2::new(0, -1);
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        ..
                    },
                ..
            } => {
                self.player.dir = Vec2::new(0, 1);
                return true;
            }
            _ => { 
                self.player.dir = Vec2::zero();
                return false;
            }
        }
        false
    }
}
