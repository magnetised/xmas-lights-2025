use crate::display::{Animate, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH};

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
    pub fn new(x: usize, y: usize) -> Box<Self> {
        let sprite = Sprite::new(&PIXELS, COLOURS.to_vec().into_iter());
        // sprite.position(x, y);
        Box::new(Square {
            points: sprite.render_at(x, y),
        })
    }
}

impl Animate for Square {
    fn step(&mut self) -> Points {
        return self.points.clone();
    }
}
