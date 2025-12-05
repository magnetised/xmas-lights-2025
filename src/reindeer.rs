use crate::display::{
    Animate, HEIGHT, HSVa, Point, Points, Rgba, Sprite, SpriteColour, WIDTH, hsv_to_rgb,
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
            r: 60f32 / 255f32,
            g: 30f32 / 255f32,
            b: 12f32 / 255f32,
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
    x: i32,
    y: i32,
    a: f32,
}

impl Reindeer {
    pub fn new(x: i32, y: i32) -> Box<Self> {
        let frame1 = Sprite::new(&FRAME_1, COLOURS.to_vec().into_iter());
        let frame2 = Sprite::new(&FRAME_2, COLOURS.to_vec().into_iter());
        Box::new(Self {
            n: 0,
            f: 0,
            frame1,
            frame2,
            x,
            y,
            a: 1.0,
        })
    }
}
impl Animate for Reindeer {
    fn step(&mut self) -> Points {
        self.n += 1;
        self.a = (self.a + 0.01) % 100.0;
        if self.n % 20 == 0 {
            self.f = (self.f + 1) % 2;
        }
        let points = if self.f == 1 {
            self.frame1.render_at(self.x, self.y)
        } else {
            self.frame2.render_at(self.x, self.y)
        };
        points
        // points
        //     .iter()
        //     .map(|p| Point {
        //         c: Rgba {
        //             a: self.a / 100.0,
        //             ..p.c
        //         },
        //         ..*p
        //     })
        //     .collect()
    }
}
