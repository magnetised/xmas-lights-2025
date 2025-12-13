use crate::display::{Animate, Points, Rgba, Sprite, SpriteColour};

#[rustfmt::skip]
const CARGO_SLEIGH: [&str; 4] = [
    " xxxxxxxxxx",
    "  xxxxxxxx ",
    "+  +    +  ",
    " ++++++++++",
];
const COLOURS: [SpriteColour; 2] = [
    (
        "x",
        Rgba {
            r: 131f32 / 255f32,
            g: 74f32 / 255f32,
            b: 72f32 / 255f32,
            a: 1.0,
        },
    ),
    (
        "+",
        Rgba {
            r: 171f32 / 255f32,
            g: 171f32 / 255f32,
            b: 171f32 / 255f32,
            a: 1.0,
        },
    ),
];
pub struct Sleigh {
    cargo: Box<dyn Animate>,
    sleigh: Sprite,
    x: i32,
    y: i32,
}

impl Sleigh {
    pub fn new(cargo: Box<dyn Animate>, x: i32, y: i32) -> Box<Self> {
        let h = cargo.height();
        let sleigh = Sprite::new_at(&CARGO_SLEIGH, &COLOURS, x, y + h as i32);
        Box::new(Self {
            cargo,
            sleigh,
            x,
            y,
        })
    }
}

impl Animate for Sleigh {
    fn step(&mut self) -> Points {
        let mut cargo = self.cargo.step();
        let mut sleigh = self.sleigh.step();
        cargo.append(&mut sleigh);
        cargo
    }

    fn width(&self) -> usize {
        // self.cargo.width().max(self.sleigh.width())
        11
    }
    fn height(&self) -> usize {
        self.cargo.height().max(self.sleigh.height())
    }
}
