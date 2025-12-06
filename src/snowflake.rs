use crate::display::{
    hsv_to_rgb, Animate, HSVa, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH,
};
use rand::prelude::*;

#[rustfmt::skip]
const FLAKE_1: [&str; 7] = [
    "  x x  ",
    " x   x ",
    "x x x x",
    "   x   ",
    "x x x x",
    " x   x ",
    "  x x  ",
];
#[rustfmt::skip]
const FLAKE_2: [&str; 9] = [
    "   x x   ",
    "  x x x  ",
    " x x x x ",
    "x x x x x",
    " x xxx x ",
    "x x x x x",
    " x x x x ",
    "  x x x  ",
    "   x x   ",
];
const COLOURS_1: [SpriteColour; 1] = [(
    "x",
    Rgba {
        r: 157.0 / 255f32,
        g: 214f32 / 255f32,
        b: 243.0 / 255f32,
        a: 0.2,
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
    w: usize,
}

impl Snowflake {
    pub fn new() -> Box<Self> {
        let sprite1 = Sprite::new(&FLAKE_1, COLOURS_1.to_vec().into_iter());
        let w = sprite1.w;
        Box::new(Self {
            flake: None,
            sprites: vec![sprite1],
            rng: Box::new(rand::rng()),
            w: w,
        })
    }
}

impl Instance {
    fn render(&self) -> Vec<Point> {
        self.sprite
            .render_at(self.x, (self.y * HEIGHT as f32).round() as i32)
    }
}
impl Animate for Snowflake {
    fn step(&mut self) -> Points {
        match &mut self.flake {
            Some(flake) => {
                flake.y += flake.vy;
                if flake.y >= ((HEIGHT as f32 + (flake.sprite.h as f32 / 2.0)) / HEIGHT as f32) {
                    self.flake = None;
                }
            }
            _ => {
                if self.rng.random::<f32>() < 0.01 {
                    let sprite = self
                        .sprites
                        .get(self.rng.random_range(0..self.sprites.len()) as usize)
                        .unwrap();

                    let flake = Instance {
                        sprite: Box::new(sprite.clone()),
                        x: self.rng.random_range(0..(WIDTH - sprite.w / 2)) as i32,
                        y: -(sprite.h as f32 / HEIGHT as f32),
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
    fn width(&self) -> usize {
        WIDTH
    }
}
