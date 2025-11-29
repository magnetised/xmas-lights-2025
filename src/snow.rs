use crate::display::{Animate, HEIGHT, Point, Rgba, WIDTH};
use rand::prelude::*;

struct Flake {
    x: usize,
    y: f32,
}

pub struct Snow {
    depth: usize,
    flakes: Vec<Flake>,
    colour: Rgba,
    vy: f32,
    rng: Box<dyn rand::RngCore>,
}

const WHITE: Rgba = Rgba {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

const MAX_FLAKES: usize = 5;
const VY: f32 = 0.018;

impl Flake {
    fn new(x: usize) -> Self {
        Flake { x: x, y: 0.0 }
    }
}

impl Snow {
    pub fn new(depth: usize) -> Box<Self> {
        let opacity = 0.7 / 1.5f32.powi(depth as i32);
        let vy = VY / 1.1f32.powi(depth as i32);
        println!("depth: {}, opacity: {}, vy: {}", depth, opacity, vy);
        Box::new(Snow {
            depth: depth,
            flakes: Vec::with_capacity(MAX_FLAKES),
            rng: Box::new(rand::rng()),
            colour: Rgba {
                a: opacity,
                ..WHITE
            },
            vy: vy,
        })
    }
    fn random_x(&mut self) -> usize {
        (self.rng.random::<f32>() * (WIDTH - 1) as f32).round() as usize
    }
}

impl Animate for Snow {
    fn step(&mut self) -> Vec<Point> {
        if self.flakes.len() < MAX_FLAKES {
            if self.rng.random::<f32>() < 0.1 {
                let flake = Flake::new(self.random_x());
                self.flakes.push(flake);
            }
        }
        // println!("l: {}", self.flakes.len());

        for flake in self.flakes.iter_mut() {
            flake.y += self.vy;
            // println!("y: {}", flake.y);
        }
        let mut indexes: Vec<usize> = Vec::with_capacity(MAX_FLAKES);
        for (i, flake) in self.flakes.iter().enumerate() {
            if flake.y >= 1.0 {
                indexes.push(i);
            }
        }
        for i in indexes.iter() {
            self.flakes.remove(*i);
        }
        self.flakes
            .iter()
            .map(|flake| {
                // println!(
                //     "x: {}, y: {}",
                //     flake.x,
                //     (flake.y * (HEIGHT - 1) as f32).round() as usize
                // );
                Point {
                    x: flake.x,
                    y: (flake.y * (HEIGHT - 1) as f32).round() as usize,
                    c: self.colour,
                }
            })
            .collect()
    }
}
