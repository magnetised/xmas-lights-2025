use crate::animation::Animation;
use crate::display::{
    darken, hsv_to_rgb, rgba, Animate, HSVa, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT,
    WIDTH,
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

const FUR: Rgba = Rgba {
    r: 121f32 / 255f32,
    g: 52f32 / 255f32,
    b: 18f32 / 255f32,
    a: 1.0,
};
const COLOURS: [SpriteColour; 4] = [
    (
        "x", // 121,52,18
        FUR,
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
    fn random_fur_colour() -> Rgba {
        let mut rng = rand::rng();
        darken(FUR, 0.5 + 0.25 * (rng.random::<f32>()))
    }
    fn random_colour(original: Rgba) -> Rgba {
        let mut rng = rand::rng();
        darken(original, 0.5 + 0.25 * (rng.random::<f32>()))
    }
    pub fn rudolf(x: i32, y: i32) -> Box<dyn Animate> {
        Self::new_with_colours(x, y, rgba(1.0, 0.0, 0.0, 1.0), FUR)
    }
    pub fn new(x: i32, y: i32) -> Box<dyn Animate> {
        let fur = Self::random_fur_colour();
        let nose = Self::random_colour(fur);
        Self::new_with_colours(x, y, nose, Self::random_fur_colour())
    }
    pub fn new_with_colours(x: i32, y: i32, nose: Rgba, fur: Rgba) -> Box<dyn Animate> {
        let colours: Vec<SpriteColour> = COLOURS
            .iter()
            .map(|colour| match colour {
                ("o", _c) => ("o", nose),
                ("x", _c) => ("x", fur),
                other => *other,
            })
            .collect();

        let frame1 = Sprite::new_at(&FRAME_1, colours.clone().into_iter(), x, y);
        let frame2 = Sprite::new_at(&FRAME_2, colours.into_iter(), x, y);

        Animation::new(vec![Box::new(frame1), Box::new(frame2)], 12)
    }
}
// impl Animate for Reindeer {
//     fn step(&mut self) -> Points {
//         self.n = (self.n + 1) % PERIOD;
//         self.a = (self.a + 0.01) % 100.0;
//         if self.n == 0 {
//             self.f = (self.f + 1) % 2;
//             // self.x += self.v;
//         }
//
//         if (self.v < 0 && self.x < -(self.w as i32)) || (self.v > 0 && self.x > WIDTH as i32) {
//             self.v = -self.v;
//             // for frame in self.frames.iter_mut() {
//             //     frame.flip();
//             // }
//         }
//         let frame = self.frames.get(self.f).unwrap();
//         let points = frame.render_at(self.x, self.y);
//         points
//     }
// }
