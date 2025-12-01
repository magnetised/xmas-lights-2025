use crate::display::{Animate, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH};

#[rustfmt::skip]

const PIXELS: [&str; 10] = [
    "  x  xxxxx",
    " xx      x",
    "  x      x",
    "  x      x",
    "  x     x ",
    "  x     x ",
    "  x    x  ",
    "  x    x  ",
    "  x    x  ",
    " xxx   x  ",
];

const COLOURS: [SpriteColour; 1] = [(
    "x",
    Rgba {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    },
)];

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
