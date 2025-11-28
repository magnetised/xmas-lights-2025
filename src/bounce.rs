use crate::display::{Animate, Point, Rgba, HEIGHT, WIDTH};
use rand::prelude::*;

pub struct Bounce {
    colour: Rgba,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Bounce {
    pub fn new(colour: Rgba, vx: f32, vy: f32) -> Box<Self> {
        let mut rng = rand::rng();
        let x = rng.random::<f32>() * (WIDTH - 1) as f32;
        let y = rng.random::<f32>() * (HEIGHT - 1) as f32;
        Box::new(Bounce {
            colour: colour,
            x: x,
            y: y,
            vx: vx,
            vy: vy,
        })
    }
    pub fn completely_random() -> Box<Self> {
        let mut rng = rand::rng();
        let r = rng.random::<f32>();
        let g = rng.random::<f32>();
        let b = rng.random::<f32>();
        let a = rng.random::<f32>();
        Bounce::random(Rgba {
            r: r,
            g: g,
            b: b,
            a: a,
        })
    }
    pub fn random(colour: Rgba) -> Box<Self> {
        let mut rng = rand::rng();
        let vx = rng.random::<f32>();
        let vy = rng.random::<f32>();

        Bounce::new(colour, vx, vy)
    }
    pub fn random_a(r: f32, g: f32, b: f32) -> Box<Self> {
        let mut rng = rand::rng();
        let a = rng.random::<f32>();
        let vx = rng.random::<f32>();
        let vy = rng.random::<f32>();

        Bounce::new(
            Rgba {
                r: r,
                g: g,
                b: b,
                a: a,
            },
            vx,
            vy,
        )
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
