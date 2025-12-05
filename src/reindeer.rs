use crate::display::{
    hsv_to_rgb, Animate, HSVa, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH,
};

#[rustfmt::skip]
const FRAME_1: [&str; 10] = [
    "y   y    ",
    "yy yy    ",
    " y y     ",
    " xxx     ",
    "oxxx    x",
    "xxxxxxxxx",
    "  xxxxxx ",
    "  xxxxxx ",
    "  x    x ",
    " . x  . x",
];

#[rustfmt::skip]
const FRAME_2: [&str; 10] = [
    "y   y    ",
    "yy yy    ",
    " y y     ",
    " xxx     ",
    "oxxx    x",
    "xxxxxxxxx",
    "  xxxxxx ",
    "  xxxxxx ",
    "  x    x ",
    " x .  x .",
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
            r: 65f32 / 255f32,
            g: 34f32 / 255f32,
            b: 17f32 / 255f32,
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

pub struct Reindeer {
    n: usize,
    f: usize,
    frame1: Sprite,
    frame2: Sprite,
    x: usize,
    y: usize,
}

impl Reindeer {
    pub fn new(x: usize, y: usize) -> Box<Self> {
        let frame1 = Sprite::new(&FRAME_1, COLOURS.to_vec().into_iter());
        let frame2 = Sprite::new(&FRAME_2, COLOURS.to_vec().into_iter());
        Box::new(Self {
            n: 0,
            f: 0,
            frame1,
            frame2,
            x,
            y,
        })
    }
}
impl Animate for Reindeer {
    fn step(&mut self) -> Points {
        self.n += 1;
        if self.n % 40 == 0 {
            self.f = (self.f + 1) % 2;
        }
        if self.f == 1 {
            self.frame1.render_at(self.x, self.y)
        } else {
            self.frame2.render_at(self.x, self.y)
        }
    }
}
