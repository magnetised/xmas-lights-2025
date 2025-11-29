use crate::display::{Animate, HEIGHT, Point, Rgba, WIDTH};
use rand::prelude::*;

struct Flake {
    x: usize,
    y: f32,
    vy: f32,
}

pub struct Snow {
    depth: usize,
    max_flakes: usize,
    flakes: Vec<Flake>,
    colour: Rgba,
    vy: f32,
    rng: Box<dyn rand::RngCore>,
}

const I: f32 = 0.5;
const WHITE: Rgba = Rgba {
    r: 0.8,
    g: 0.8,
    b: 0.8,
    a: 1.0,
};

const VY: f32 = 0.020;

impl Flake {
    fn new(x: usize, vy: f32) -> Self {
        Flake { x, y: 0.0, vy }
    }
}

impl Snow {
    pub fn new(depth: usize, max: usize) -> Box<Self> {
        // let i = I / 1.45f32.powi(depth as i32);
        let i = I / (1.1 * depth as f32);
        let vy = VY / 1.1f32.powf(depth as f32 - 1.0);
        // let vy = VY / (0.2 * depth as f32);
        println!("depth: {}, i: {}, vy: {}", depth, i, vy);
        Box::new(Snow {
            depth,
            max_flakes: max,
            flakes: Vec::with_capacity(max),
            rng: Box::new(rand::rng()),
            colour: Rgba {
                r: i,
                g: i,
                b: i,
                a: 1.0,
            },
            vy,
        })
    }
    fn random_x(&mut self) -> usize {
        self.rng.random_range(0..WIDTH) as usize
    }
    fn random_vy(&mut self) -> f32 {
        self.vy - (self.vy * 0.6 * self.rng.random::<f32>())
    }
}

impl Animate for Snow {
    fn step(&mut self) -> Vec<Point> {
        if self.flakes.len() < self.max_flakes {
            if self.rng.random::<f32>() < 0.10 {
                let flake = Flake::new(self.random_x(), self.random_vy());
                self.flakes.push(flake);
            }
        }

        for flake in self.flakes.iter_mut() {
            flake.y += flake.vy;
        }
        self.flakes.retain(|flake| flake.y < 1.0);
        self.flakes
            .iter()
            .map(|flake| Point {
                x: flake.x,
                y: (flake.y * (HEIGHT - 1) as f32).round() as usize,
                c: self.colour,
            })
            .collect()
    }
}
