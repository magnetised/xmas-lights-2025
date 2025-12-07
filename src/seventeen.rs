use crate::display::{
    hsv_to_rgb, Animate, HSVa, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH,
};

#[rustfmt::skip]
const LARGE: [&str; 10] = [
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

#[rustfmt::skip]
const SMALL: [&str; 6] = [
    " xx xxxx",
    "xxx xxxx",
    " xx   xx",
    " xx  xx ",
    " xx  xx ",
    "xxxx xx ",
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
    hue: f32,
    w: usize,
    h: usize,
}

impl Seventeen {
    pub fn large(x: i32, y: i32) -> Box<Self> {
        let sprite = Sprite::new(&LARGE, COLOURS.to_vec().into_iter());
        Box::new(Seventeen {
            points: sprite.render_at(x, y),
            hue: 0.0,
            w: sprite.w,
            h: sprite.h,
        })
    }
    pub fn small(x: i32, y: i32) -> Box<Self> {
        let sprite = Sprite::new(&SMALL, COLOURS.to_vec().into_iter());
        Box::new(Seventeen {
            points: sprite.render_at(x, y),
            hue: 0.0,
            w: sprite.w,
            h: sprite.h,
        })
    }
}

impl Animate for Seventeen {
    fn step(&mut self) -> Points {
        self.hue = (self.hue + 0.5) % 360.0;
        let hsva = HSVa {
            h: self.hue,
            s: 1.0,
            v: 0.5,
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
    fn width(&self) -> usize {
        self.w
    }
    fn height(&self) -> usize {
        self.h
    }
}
