use crate::display::{Animate, Points, Rgba, Sprite, SpriteColour};

#[rustfmt::skip]
const PIXELS: [&str; 3] = [
    "xxxx",
    "xxxx",
    "xxxx"
];
const COLOURS: [SpriteColour; 1] = [(
    "x",
    Rgba {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
)];

pub struct Square {
    points: Points,
}

impl Square {
    pub fn new(x: i32, y: i32) -> Box<Self> {
        let sprite = Sprite::new(&PIXELS, &COLOURS);
        Box::new(Square {
            points: sprite.render_at(x, y),
        })
    }
}

impl Animate for Square {
    fn step(&mut self) -> Points {
        return self.points.clone();
    }
    fn width(&self) -> usize {
        4
    }
    fn height(&self) -> usize {
        3
    }
}
