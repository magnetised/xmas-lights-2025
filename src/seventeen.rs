use crate::display::{hsv_to_rgb, Animate, Group, HSVa, Point, Points, Rgba, Sprite, SpriteColour};

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
#[rustfmt::skip]
const STRIPY_1: [&str; 8] = [
    "          ",
    "          ",
    "          ",
    "          ",
    "          ",
    "          ",
    " xx       ",
    "          ",
];
#[rustfmt::skip]
const STRIPY_2: [&str; 8] = [
    "          ",
    "          ",
    "          ",
    "          ",
    "  x       ",
    "  xx      ",
    "   xx     ",
    "          ",
];
#[rustfmt::skip]
const STRIPY_3: [&str; 8] = [
    "          ",
    "          ",
    " xx       ",
    "  xx      ",
    "   x      ",
    "          ",
    "      x   ",
    "          ",
];
#[rustfmt::skip]
const STRIPY_4: [&str; 8] = [
    "          ",
    "  xx      ",
    "   x      ",
    "          ",
    "      x   ",
    "      xx  ",
    "       x  ",
    "          ",
];
#[rustfmt::skip]
const STRIPY_5: [&str; 8] = [
    "          ",
    "     x    ",
    "     xx   ",
    "       x  ",
    "       x  ",
    "          ",
    "          ",
    "          ",
];
#[rustfmt::skip]
const STRIPY_6: [&str; 8] = [
    "          ",
    "      xx  ",
    "       xx ",
    "        x ",
    "          ",
    "          ",
    "          ",
    "          ",
];
#[rustfmt::skip]
const STRIPY_7: [&str; 8] = [
    "          ",
    "        x ",
    "          ",
    "          ",
    "          ",
    "          ",
    "          ",
    "          ",
];
#[rustfmt::skip]
const BORDER: [&str; 8] = [
    "..........",
    ".        .",
    ".        .",
    ".        .",
    ".        .",
    ".        .",
    ".        .",
    "..........",

];

const COLOURS: [SpriteColour; 2] = [
    (
        "x",
        Rgba {
            r: 0.0,
            g: 1.0,
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
];

pub struct Seventeen {
    points: Points,
    hue: f32,
    w: usize,
    h: usize,
}

impl Seventeen {
    pub fn large(x: i32, y: i32) -> Box<Self> {
        let sprite = Sprite::new(&LARGE, &COLOURS);
        Box::new(Seventeen {
            points: sprite.render_at(x, y),
            hue: 0.0,
            w: sprite.w,
            h: sprite.h,
        })
    }
    pub fn small(x: i32, y: i32) -> Box<Self> {
        let sprite = Sprite::new(&SMALL, &COLOURS);
        Box::new(Seventeen {
            points: sprite.render_at(x, y),
            hue: 0.0,
            w: sprite.w,
            h: sprite.h,
        })
    }
    pub fn stripy(x: i32, y: i32) -> Box<dyn Animate> {
        let sprite_1 = Sprite::new(&STRIPY_1, &COLOURS);
        let sprite_2 = Sprite::new(&STRIPY_2, &COLOURS);
        let sprite_3 = Sprite::new(&STRIPY_3, &COLOURS);
        let sprite_4 = Sprite::new(&STRIPY_4, &COLOURS);
        let sprite_5 = Sprite::new(&STRIPY_5, &COLOURS);
        let sprite_6 = Sprite::new(&STRIPY_6, &COLOURS);
        let sprite_7 = Sprite::new(&STRIPY_7, &COLOURS);
        let border = Sprite::new_at(&BORDER, &COLOURS, x, y);

        // Box::new(Seventeen {
        //     points: sprite.render_at(x, y),
        //     hue: 0.0,
        //     w: sprite.w,
        //     h: sprite.h,
        // })
        Group::new(vec![
            Box::new(Seventeen {
                points: sprite_1.render_at(x, y),
                hue: 0.0,
                w: sprite_1.w,
                h: sprite_1.h,
            }),
            Box::new(Seventeen {
                points: sprite_2.render_at(x, y),
                hue: 45.0,
                w: sprite_2.w,
                h: sprite_2.h,
            }),
            Box::new(Seventeen {
                points: sprite_3.render_at(x, y),
                hue: 90.0,
                w: sprite_3.w,
                h: sprite_3.h,
            }),
            Box::new(Seventeen {
                points: sprite_4.render_at(x, y),
                hue: 135.0,
                w: sprite_4.w,
                h: sprite_4.h,
            }),
            Box::new(Seventeen {
                points: sprite_5.render_at(x, y),
                hue: 180.0,
                w: sprite_5.w,
                h: sprite_5.h,
            }),
            Box::new(Seventeen {
                points: sprite_6.render_at(x, y),
                hue: 225.0,
                w: sprite_6.w,
                h: sprite_6.h,
            }),
            Box::new(Seventeen {
                points: sprite_7.render_at(x, y),
                hue: 270.0,
                w: sprite_7.w,
                h: sprite_7.h,
            }),
            Box::new(border),
        ])
    }
}

impl Animate for Seventeen {
    fn step(&mut self) -> Points {
        self.hue = (self.hue + 1.8) % 360.0;
        let hsva = HSVa {
            h: self.hue,
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
    fn width(&self) -> usize {
        self.w
    }
    fn height(&self) -> usize {
        self.h
    }
}
