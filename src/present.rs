use crate::display::{Animate, Points, Rgba, Sprite, SpriteColour};
use std::vec::IntoIter;

#[rustfmt::skip]
const LARGE: [&str; 5] = [
    "xx0xx",
    "xx0xx",
    "00000",
    "xx0xx",
    "xx0xx",
];
#[rustfmt::skip]
const SMALL: [&str; 3] = [
    "x0x",
    "000",
    "x0x",
];

const COLOURS: [SpriteColour; 2] = [
    (
        "x",
        Rgba {
            r: 0.8,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    ),
    (
        "0",
        Rgba {
            r: 0.8,
            g: 0.8,
            b: 0.0,
            a: 1.0,
        },
    ),
];

pub struct Present {
    points: Points,
    w: usize,
}

impl Present {
    fn colours<'a>(box_colour: Rgba, ribbon_colour: Rgba) -> IntoIter<SpriteColour<'a>> {
        vec![("x", box_colour), ("0", ribbon_colour)].into_iter()
    }

    pub fn large(x: i32, y: i32, box_color: Rgba, ribbon_colour: Rgba) -> Box<Self> {
        let sprite = Sprite::new(&LARGE, Present::colours(box_color, ribbon_colour));
        Box::new(Present {
            points: sprite.render_at(x, y),
            w: sprite.width(),
        })
    }
    pub fn small(x: i32, y: i32, box_color: Rgba, ribbon_colour: Rgba) -> Box<Self> {
        let sprite = Sprite::new(&SMALL, Present::colours(box_color, ribbon_colour));
        Box::new(Present {
            points: sprite.render_at(x, y),
            w: sprite.width(),
        })
    }
}

impl Animate for Present {
    fn step(&mut self) -> Points {
        return self.points.clone();
    }
    fn width(&self) -> usize {
        self.w
    }
}
