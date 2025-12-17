use crate::animation::Animation;
use crate::display::{
    darken, hsv_to_rgb, rgba, Animate, Group, HSVa, Point, Points, Rgba, Sprite, SpriteColour,
    HEIGHT, WIDTH,
};
use rand::prelude::*;

#[rustfmt::skip]
const FRAME_0: &[&str] = &[
    "     ",
    "     ",
    "  o  ",
    "     ",
    "     ",
];

#[rustfmt::skip]
const FRAME_1: &[&str] = &[
    "     ",
    "  o  ",
    " o o ",
    "  o  ",
    "     ",
];
#[rustfmt::skip]
const FRAME_2: &[&str] = &[
    "  o  ",
    "     ",
    "     ",
    "     ",
    "  o  ",
];

const SPRITE_COLOURS: [SpriteColour; 1] = [(
    "o",
    Rgba {
        r: 213f32 / 255f32,
        g: 27f32 / 255f32,
        b: 52f32 / 255f32,
        a: 1.0,
    },
)];

const COLOURS: &[Rgba] = &[
    Rgba {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    },
    Rgba {
        r: 0.5,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    },
    Rgba {
        r: 0.3,
        g: 0.3,
        b: 0.0,
        a: 1.0,
    },
];
struct Layer {
    sprite: Sprite,
    n: usize,
    c: usize,
    d: i32,
}

pub struct Star {
    layers: Vec<Layer>,
}
pub struct Speck {
    x: i32,
    y: i32,
    b: f32,
    rng: ThreadRng,
}

impl Star {
    pub fn new(x: i32, y: i32) -> Box<dyn Animate> {
        let layers = vec![
            Layer {
                sprite: Sprite::new_at(FRAME_0, &SPRITE_COLOURS, x, y),
                n: 0,
                c: 2,
                d: 1,
            },
            Layer {
                sprite: Sprite::new_at(FRAME_1, &SPRITE_COLOURS, x, y),
                n: 0,
                c: 1,
                d: 1,
            },
            Layer {
                sprite: Sprite::new_at(FRAME_2, &SPRITE_COLOURS, x, y),
                n: 0,
                c: 0,
                d: 1,
            },
        ];
        Box::new(Star { layers })
    }
    pub fn sprinkle(n: usize) -> Box<dyn Animate> {
        let mut stars: Vec<Box<dyn Animate>> = Vec::with_capacity(n);
        let mut rng = rand::rng();
        for _i in 0..n {
            let s = Speck {
                x: rng.random_range(0..WIDTH) as i32,
                y: rng.random_range(0..HEIGHT) as i32,
                b: 100.0 * rng.random::<f32>(),
                rng: rand::rng(),
            };
            stars.push(Box::new(s));
        }

        Group::new(stars)
    }
}

impl Animate for Speck {
    fn step(&mut self) -> Vec<Point> {
        self.b = (self.b + 8.0 * (2.0 * self.rng.random::<f32>() - 1.0)) % 100.0;
        let c = hsv_to_rgb(HSVa {
            h: 54.0,
            s: 1.0,
            v: 0.5 + (self.b / 250.0),
            a: 1.0,
        });
        vec![Point {
            x: self.x,
            y: self.y,
            c,
        }]
    }
    fn width(&self) -> usize {
        self.x as usize + 1
    }
    fn height(&self) -> usize {
        self.y as usize + 1
    }
}
impl Animate for Star {
    fn step(&mut self) -> Points {
        self.layers
            .iter_mut()
            .flat_map(|layer| layer.step())
            .collect()
    }
    fn width(&self) -> usize {
        FRAME_0[0].len()
    }
    fn height(&self) -> usize {
        FRAME_0.len()
    }
}

impl Animate for Layer {
    fn step(&mut self) -> Vec<Point> {
        self.n = (self.n + 1) % 16;
        if self.n == 0 {
            self.c = (self.c as i32 + self.d) as usize % COLOURS.len();
            if self.c == 0 {
                // self.d = -self.d;
            }
        }

        let colour = COLOURS[self.c];

        self.sprite
            .step()
            .iter()
            .map(|p| Point { c: colour, ..*p })
            .collect()
    }
    fn width(&self) -> usize {
        self.sprite.width()
    }
    fn height(&self) -> usize {
        self.sprite.height()
    }
}
