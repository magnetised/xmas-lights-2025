use crate::display::{
    hsv_to_rgb, Animate, HSVa, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH,
};
use rand::prelude::*;

#[rustfmt::skip]
const FLAKE_1: [&str; 5] = [
    " x   x ",
    "x x x x",
    "   x   ",
    "x x x x",
    " x   x ",
];
const COLOURS_1: [SpriteColour; 1] = [(
    "x",
    // 121,52,18
    Rgba {
        r: 0.0 / 255f32,
        g: 255f32 / 255f32,
        b: 0.0 / 255f32,
        a: 1.0,
    },
)];
const VY: f32 = 0.010;

#[derive(Clone, Debug)]
struct Instance {
    sprite: Box<Sprite>,
    x: i32,
    y: f32,
    vy: f32,
}

pub struct Snowflake {
    flake: Option<Instance>,
    sprites: Vec<Sprite>,
    rng: Box<dyn rand::RngCore>,
}

impl Snowflake {
    pub fn new() -> Box<Self> {
        let sprite1 = Sprite::new(&FLAKE_1, COLOURS_1.to_vec().into_iter());
        Box::new(Self {
            flake: None,
            sprites: vec![sprite1],
            rng: Box::new(rand::rng()),
        })
    }
}

impl Instance {
    fn render(&self) -> Vec<Point> {
        println!("flake: {} {}", self.x, (self.y * HEIGHT as f32) as usize);
        self.sprite
            .render_at(self.x, (self.y * HEIGHT as f32) as i32)
    }
}
impl Animate for Snowflake {
    fn step(&mut self) -> Points {
        match &mut self.flake {
            Some(flake) => {
                println!("y: {}, vy: {}", flake.y, flake.vy);
                flake.y += flake.vy;
                if flake.y >= 1.0 {
                    self.flake = None;
                }
            }
            _ => {
                println!("NO FLAKE");
                if self.rng.random::<f32>() < 0.01 {
                    println!("NEW FLAKE");
                    println!("new flake");
                    let sprite = self
                        .sprites
                        .get(self.rng.random_range(0..self.sprites.len()) as usize)
                        .unwrap();
                    let flake = Instance {
                        sprite: Box::new(sprite.clone()),
                        x: self.rng.random_range(0..(WIDTH - 3)) as i32,
                        y: -(3.0 / HEIGHT as f32),
                        vy: VY - (VY * 0.6 * self.rng.random::<f32>()),
                    };
                    self.flake = Some(flake);
                }
            }
        };
        match &self.flake {
            Some(flake) => flake.render().clone(),
            _ => vec![],
        }
    }
}
