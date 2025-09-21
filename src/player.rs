use macroquad::prelude::*;

#[allow(dead_code)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub health: i32,
    pub health_max: i32,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            width: 50.0,
            height: 50.0,
            speed: 200.0,
            health: 100,
            health_max: 100,
            inventory: vec![],
        }
    }

    pub fn update(&mut self, dt: f32, world_width: f32, world_height: f32) {
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dy -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dy += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dx -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dx += 1.0;
        }

        // Normalize diagonal movement
        if dx != 0.0 && dy != 0.0 {
            let mag = (dx*dx + dy*dy).sqrt();
            dx /= mag;
            dy /= mag;
        }

        self.x += dx * self.speed * dt;
        self.y += dy * self.speed * dt;

        // Clamp to world bounds
        self.x = self.x.clamp(0.0, world_width - self.width);
        self.y = self.y.clamp(0.0, world_height - self.height);
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, BLUE);
    }
}