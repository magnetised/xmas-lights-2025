use crate::display::{
    hsv_to_rgb, Animate, HSVa, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH,
};

#[rustfmt::skip]

const PIXELS: [&str; 10] = [
    "  xx  xxxxx",
    " xxx  xxxxx",
    "  xx     xx",
    "  xx     xx",
    "  xx    xx ",
    "  xx    xx ",
    "  xx    xx ",
    "  xx   xx  ",
    " xxxx  xx  ",
    " xxxx  xx  ",
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
    h: f32,
}

impl Seventeen {
    pub fn new(x: usize, y: usize) -> Box<Self> {
        let sprite = Sprite::new(&PIXELS, &COLOURS);
        Box::new(Seventeen {
            points: sprite.render_at(x, y),
            h: 0.0,
        })
    }
}

impl Animate for Seventeen {
    fn step(&mut self) -> Points {
        self.h += 0.5;
        let hsva = HSVa {
            h: self.h,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let rgba = hsv_to_rgb(hsva);
        let p: Points = self
            .points
            .iter()
            .map(|p| Point { c: rgba, ..*p })
            .collect();
        return p;
    }
}
