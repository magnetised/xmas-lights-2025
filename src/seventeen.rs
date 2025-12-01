use crate::display::{Animate, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH};

#[rustfmt::skip]

const PIXELS: [&str; 10] = [
    "  xx  xxxxx",
    " xxx  xxxxx",
    "  xx     xx",
    "  xx     x*",
    "  xx    *x ",
    "  xx    xx ",
    "  xx    x* ",
    "  xx   *x  ",
    " xxxx  xx  ",
    " xxxx  xx  ",
];

const COLOURS: [SpriteColour; 2] = [
    (
        "x",
        Rgba {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        },
    ),
    (
        "*",
        Rgba {
            r: 0.0,
            g: 0.7,
            b: 0.0,
            a: 1.0,
        },
    ),
];

pub struct Seventeen {
    points: Points,
}

impl Seventeen {
    pub fn new(x: usize, y: usize) -> Box<Self> {
        let sprite = Sprite::new(&PIXELS, &COLOURS);
        Box::new(Seventeen {
            points: sprite.render_at(x, y),
        })
    }
}

impl Animate for Seventeen {
    fn step(&mut self) -> Points {
        return self.points.clone();
    }
}
