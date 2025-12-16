use crate::animation::Animation;
use crate::display::{darken, rgba, Animate, Rgba, Sprite, SpriteColour, WHITE};
use crate::sleigh;

#[rustfmt::skip]
const FRAME_0: [&str; 13] = [
    "      b    ",
    "      o    ",
    "     ooo   ",
    "    offf   ",
    "    obfb   ",
    "    obbb w ",
    " w  oooooww",
    "w w ==x==ww",
    "w   oooooww",
    "wwwwwwwwwww",
    " wwwwwwwww ",
    "+  +    +  ",
    " ++++++++++",
];
#[rustfmt::skip]
const FRAME_1: [&str; 13] = [
    "       b   ",
    "      o    ",
    "     ooo   ",
    "   o fff   ",
    "   o bfb   ",
    "    obbb w ",
    " w  oooooww",
    "w w ==x==ww",
    "w   oooooww",
    "wwwwwwwwwww",
    " wwwwwwwww ",
    "+  +    +  ",
    " ++++++++++",
];
#[rustfmt::skip]
const FRAME_2: [&str; 13] = [
    "           ",
    "      ob   ",
    "     ooo   ",
    "  o  fff   ",
    "   o bfb   ",
    "    obbb w ",
    " w  oooooww",
    "w w ==x==ww",
    "w   oooooww",
    "wwwwwwwwwww",
    " wwwwwwwww ",
    "+  +    +  ",
    " ++++++++++",
];

const COLOURS: [SpriteColour; 7] = [
    ("w", sleigh::SLEIGH_COLOUR),
    (
        "o",
        Rgba {
            r: 213f32 / 255f32,
            g: 27f32 / 255f32,
            b: 52f32 / 255f32,
            a: 1.0,
        },
    ),
    ("b", WHITE),
    (
        "f",
        Rgba {
            r: 254f32 / 255f32,
            g: 224f32 / 255f32,
            b: 205f32 / 255f32,
            a: 1.0,
        },
    ),
    ("+", sleigh::RUNNER_COLOUR),
    (
        "=",
        Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    ),
    (
        "x",
        Rgba {
            r: 233f32 / 255f32,
            g: 183f32 / 255f32,
            b: 127f32 / 255f32,
            a: 1.0,
        },
    ),
];

pub struct Santa {}

impl Santa {
    pub fn new(x: i32, y: i32) -> Box<dyn Animate> {
        Animation::new_with_frames(
            &[&FRAME_0, &FRAME_1, &FRAME_2, &FRAME_1],
            &COLOURS,
            12,
            x,
            y,
        )
    }
}
