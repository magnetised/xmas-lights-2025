use crate::display::{
    Animate, HEIGHT, HSVa, Point, Points, Rgba, Sprite, SpriteColour, WIDTH, hsv_to_rgb,
};
use rand::prelude::*;

#[rustfmt::skip]
const FRAME_1: [&str; 10] = [
    "y   y     ",
    "yy yy     ",
    " y y      ",
    " xxx      ",
    "oxxx    x ",
    "xxxxxxxxx ",
    "  xxxxxx  ",
    "  xxxxxx  ",
    "  x    x  ",
    " . x  . x ",
];

#[rustfmt::skip]
const FRAME_2: [&str; 10] = [
    "y   y     ",
    "yy yy     ",
    " y y      ",
    " xxx      ",
    "oxxx      ",
    "xxxxxxxxxx",
    "  xxxxxx  ",
    "  xxxxxx  ",
    "  x    x  ",
    " x .  x . ",
];

const COLOURS: [SpriteColour; 4] = [
    (
        "x",
        // 121,52,18
        Rgba {
            r: 121f32 / 255f32,
            g: 52f32 / 255f32,
            b: 18f32 / 255f32,
            a: 1.0,
        },
    ),
    (
        ".",
        // 121,52,18
        Rgba {
            r: 30f32 / 255f32,
            g: 15f32 / 255f32,
            b: 5f32 / 255f32,
            a: 1.0,
        },
    ),
    (
        "y",
        Rgba {
            // 178,108,47
            r: 178f32 / 255f32,
            g: 108f32 / 255f32,
            b: 47f32 / 255f32,
            a: 1.0,
        },
    ),
    (
        "o",
        // 255,0,0
        Rgba {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    ),
];

const PERIOD: usize = 12;
pub struct Reindeer {
    n: usize,
    f: usize,
    frames: Vec<Sprite>,
    x: i32,
    y: i32,
    a: f32,
    v: i32,
    w: usize,
}

impl Reindeer {
    pub fn new(x: i32, y: i32) -> Box<Self> {
        let frame1 = Sprite::new(&FRAME_1, COLOURS.to_vec().into_iter());
        let frame2 = Sprite::new(&FRAME_2, COLOURS.to_vec().into_iter());
        let w = frame1.w;
        let mut rng = rand::rng();
        Box::new(Self {
            n: rng.random_range(0..PERIOD as usize),
            f: 0,
            frames: vec![frame1, frame2],
            x,
            y,
            a: 1.0,
            v: -1,
            w: w,
        })
    }
}
impl Animate for Reindeer {
    fn step(&mut self) -> Points {
        self.n = (self.n + 1) % PERIOD;
        self.a = (self.a + 0.01) % 100.0;
        if self.n == 0 {
            self.f = (self.f + 1) % 2;
            self.x += self.v;
        }

        if (self.v < 0 && self.x < -(self.w as i32)) || (self.v > 0 && self.x > WIDTH as i32) {
            self.v = -self.v;
            for frame in self.frames.iter_mut() {
                frame.flip();
            }
        }
        let frame = self.frames.get(self.f).unwrap();
        let points = frame.render_at(self.x, self.y);
        points
    }
}
