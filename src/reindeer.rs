use crate::display::{
    hsv_to_rgb, Animate, HSVa, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH,
};

#[rustfmt::skip]
const PIXELS: [&str; 10] = [
    "a   a    ",
    "aa aa    ",
    " a a     ",
    " rrr     ",
    "nrrr    r",
    "rrrrrrrrr",
    "  rrrrrr ",
    "  rrrrrr ",
    "  r    r ",
    " rr   rr ",
];
const COLOURS: [SpriteColour; 3] = [
    (
        "r",
        // 121,52,18
        Rgba {
            r: 121f32 / 255f32,
            g: 52f32 / 255f32,
            b: 18f32 / 255f32,
            a: 1.0,
        },
    ),
    (
        "a",
        Rgba {
            // 178,108,47
            r: 178f32 / 255f32,
            g: 108f32 / 255f32,
            b: 47f32 / 255f32,
            a: 1.0,
        },
    ),
    (
        "n",
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
    sprite: Sprite,
    x: usize,
    y: usize,
}

impl Reindeer {
    pub fn new(x: usize, y: usize) -> Box<Self> {
        let sprite = Sprite::new(&PIXELS, COLOURS.to_vec().into_iter());
        Box::new(Self {
            sprite: sprite,
            x,
            y,
        })
    }
}
impl Animate for Reindeer {
    fn step(&mut self) -> Points {
        self.sprite.render_at(self.x, self.y)
    }
}
