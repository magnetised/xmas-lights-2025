use crate::animation::Animation;
use crate::display::{darken, rgba, Animate, Rgba, Sprite, SpriteColour};
use rand::prelude::*;

#[rustfmt::skip]
const FRAME_0: [&str; 13] = [
    "        b  ",
    "       o   ",
    "      ooo  ",
    "     offf  ",
    "     obfb  ",
    "     obbbw ",
    " ww  ooooww",
    "w  w ooooww",
    "w    ooooww",
    "wwwwwwwwwww",
    " wwwwwwwww ",
    "+  +    +  ",
    " ++++++++++",
];
#[rustfmt::skip]
const FRAME_1: [&str; 13] = [
    "        b  ",
    "       o   ",
    "      ooo  ",
    "   o  fff  ",
    "   o  bfb  ",
    "    o bbbw ",
    " ww  ooooww",
    "w  w ooooww",
    "w    ooooww",
    "wwwwwwwwwww",
    " wwwwwwwww ",
    "+  +    +  ",
    " ++++++++++",
];

#[rustfmt::skip]
const FRAME_2: [&str; 13] = [
    "      b    ",
    "       o   ",
    "      ooo  ",
    "  o   fff  ",
    "   o  bfb  ",
    "    o bbbw ",
    " ww  ooooww",
    "w  w ooooww",
    "w    ooooww",
    "wwwwwwwwwww",
    " wwwwwwwww ",
    "+  +    +  ",
    " ++++++++++",
];
#[rustfmt::skip]
const FRAME_3: [&str; 13] = [
    "        b  ",
    "       o   ",
    "      ooo  ",
    "   o  fff  ",
    "   o  bfb  ",
    "    o bbbw ",
    " ww  ooooww",
    "w  w ooooww",
    "w    ooooww",
    "wwwwwwwwwww",
    " wwwwwwwww ",
    "+  +    +  ",
    " ++++++++++",
];

const COLOURS: [SpriteColour; 5] = [
    (
        "w",
        Rgba {
            r: 131f32 / 255f32,
            g: 74f32 / 255f32,
            b: 72f32 / 255f32,
            a: 1.0,
        },
    ),
    (
        "o",
        Rgba {
            r: 213f32 / 255f32,
            g: 27f32 / 255f32,
            b: 52f32 / 255f32,
            a: 1.0,
        },
    ),
    (
        "b",
        Rgba {
            // 178,108,47
            r: 255f32 / 255f32,
            g: 255f32 / 255f32,
            b: 255f32 / 255f32,
            a: 1.0,
        },
    ),
    (
        "f",
        Rgba {
            r: 254f32 / 255f32,
            g: 224f32 / 255f32,
            b: 205f32 / 255f32,
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

pub struct Santa {}

impl Santa {
    pub fn new(x: i32, y: i32) -> Box<dyn Animate> {
        let frame1 = Sprite::new_at(&FRAME_0, &COLOURS, x, y);
        let frame2 = Sprite::new_at(&FRAME_1, &COLOURS, x, y);
        let frame3 = Sprite::new_at(&FRAME_2, &COLOURS, x, y);
        let frame4 = Sprite::new_at(&FRAME_3, &COLOURS, x, y);

        Animation::new(
            vec![
                Box::new(frame1),
                Box::new(frame2),
                Box::new(frame3),
                Box::new(frame4),
            ],
            12,
        )
    }
}
