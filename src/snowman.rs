use crate::animation::Animation;
use crate::display::{darken, rgba, Animate, Rgba, Sprite, SpriteColour, WHITE};
use rand::prelude::*;

#[rustfmt::skip]
const FRAME_1: [&str; 13] = [
    "  o      ",
    "   r     ",
    "  rrr    ",
    "  ooo    ",
    " o.o.o  r",
    " oovoo r ",
    "  ooo r  ",
    "  rrrr   ",
    " oo.oo   ",
    "ooooooo  ",
    "ooo.ooo  ",
    "ooooooo  ",
    " ooooo   ",
];
#[rustfmt::skip]
const FRAME_2: [&str; 13] = [
    "    o    ",
    "   r     ",
    "  rrr    ",
    "  ooo    ",
    " o.o.o   ",
    " oovoo   ",
    "  ooo rrr",
    "  rrrr   ",
    " oo.oo   ",
    "ooooooo  ",
    "ooo.ooo  ",
    "ooooooo  ",
    " ooooo   ",
];
#[rustfmt::skip]
const FRAME_3: [&str; 13] = [
    "    o    ",
    "   r     ",
    "  rrr    ",
    "  ooo    ",
    " o.o.o   ",
    " oovoo   ",
    "  ooo    ",
    "  rrrrrrr",
    " oo.oo   ",
    "ooooooo  ",
    "ooo.ooo  ",
    "ooooooo  ",
    " ooooo   ",
];
#[rustfmt::skip]
const FRAME_4: [&str; 13] = [
    "  o      ",
    "   r     ",
    "  rrr    ",
    "  ooo    ",
    " o.o.o   ",
    " oovoo   ",
    "  ooo    ",
    "  rrrr   ",
    " oo.oorrr",
    "ooooooo  ",
    "ooo.ooo  ",
    "ooooooo  ",
    " ooooo   ",
];
#[rustfmt::skip]
const FRAME_5: [&str; 13] = [
    "  o      ",
    "   r     ",
    "  rrr    ",
    "  ooo    ",
    " o.o.o   ",
    " oovoo   ",
    "  ooo    ",
    "  rrrr   ",
    " oo.oor  ",
    "ooooooor ",
    "ooo.ooo r",
    "ooooooo  ",
    " ooooo   ",
];

const COLOURS: [SpriteColour; 4] = [
    (
        "v",
        Rgba {
            r: 255f32 / 255f32,
            g: 138f32 / 255f32,
            b: 0.0,
            a: 1.0,
        },
    ),
    (
        ".",
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    ),
    (
        "r",
        Rgba {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    ),
    ("o", WHITE),
];

pub struct Snowman {}

impl Snowman {
    pub fn new(x: i32, y: i32) -> Box<dyn Animate> {
        Animation::new_with_frames(
            &[
                &FRAME_1, &FRAME_2, &FRAME_3, &FRAME_4, &FRAME_5, &FRAME_4, &FRAME_3, &FRAME_2,
            ],
            &COLOURS,
            12,
            x,
            y,
        )
    }
}
