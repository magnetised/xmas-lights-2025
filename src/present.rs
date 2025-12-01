use crate::display::{Animate, Points, Rgba, Sprite, SpriteColour};

#[rustfmt::skip]
const PIXELS: [&str; 5] = [
    "xx0xx",
    "xx0xx",
    "00000",
    "xx0xx",
    "xx0xx",
];

const COLOURS: [SpriteColour; 2] = [
    (
        "x",
        Rgba {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    ),
    (
        "0",
        Rgba {
            r: 1.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        },
    ),
];

pub struct Present {
    points: Points,
}

impl Present {
    pub fn new(x: usize, y: usize) -> Box<Self> {
        let sprite = Sprite::new(&PIXELS, &COLOURS);
        Box::new(Present {
            points: sprite.render_at(x, y),
        })
    }
}

impl Animate for Present {
    fn step(&mut self) -> Points {
        return self.points.clone();
    }
}
