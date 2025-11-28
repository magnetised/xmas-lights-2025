use crate::display::{Animate, Point, Rgba, HEIGHT, WIDTH};

pub struct Bounce {
    colour: Rgba,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Bounce {
    pub fn new(colour: Rgba, vx: f32, vy: f32) -> Self {
        Bounce {
            colour: colour,
            x: 0.0,
            y: 0.0,
            vx: vx,
            vy: vy,
        }
    }
}

impl Animate for Bounce {
    fn step(&mut self) -> Vec<Point> {
        self.x += self.vx;
        self.y += self.vy;

        if self.x > (WIDTH - 1) as f32 {
            self.x = (WIDTH - 1) as f32;
            self.vx = -self.vx;
        }
        if self.y > (HEIGHT - 1) as f32 {
            self.y = (HEIGHT - 1) as f32;
            self.vy = -self.vy;
        }
        if self.x < 0.0 {
            self.x = 0.0;
            self.vx = -self.vx;
        }
        if self.y < 0.0 {
            self.y = 0.0;
            self.vy = -self.vy;
        }

        return vec![Point {
            x: self.x.round() as usize,
            y: self.y.round() as usize,
            c: self.colour,
        }];
    }
}
