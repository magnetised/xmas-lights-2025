use crate::animation::Animation;
use crate::display::{darken, rgba, Animate, Group, Point, Points, Rgba, Sprite, SpriteColour};
use rand::prelude::*;

#[rustfmt::skip]
const FRAME_1: [&str; 12] = [
    "    o    ",
    "   ooo   ",
    "    o    ",
    "   xxx   ",
    "  xxxxx  ",
    "   xxx   ",
    "  xxxxx  ",
    " xxxxxxx ",
    "  xxxxx  ",
    " xxxxxxx ",
    "xxxxxxxxx",
    "         ",
];
const LIGHTS: [&str; 11] = [
    "         ",
    "         ",
    "         ",
    "     .   ",
    "  .      ",
    "     .   ",
    "    .    ",
    " . .   . ",
    "     .   ",
    "  . .    ",
    " .    .  ",
];

const COLOURS: [SpriteColour; 3] = [
    (
        "x",
        Rgba {
            r: 0.0,
            g: 255f32 / 255f32,
            b: 0.0,
            a: 1.0,
        },
    ),
    (
        "o",
        Rgba {
            r: 255f32 / 255f32,
            g: 255f32 / 255f32,
            b: 0.0,
            a: 1.0,
        },
    ),
    (
        ".",
        Rgba {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
    ),
];

const LIGHT_COLOURS: [Rgba; 5] = [
    Rgba {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
    Rgba {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    },
    Rgba {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    },
    Rgba {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    },
    Rgba {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
];

fn random_light_colour(rng: &mut ThreadRng) -> usize {
    rng.random_range(0..LIGHT_COLOURS.len()) as usize
}
#[derive(Clone, Debug)]
struct Light {
    point: Point,
    colour: usize,
    rng: ThreadRng,
}

pub struct Tree {
    sprite: Sprite,
    lights: Vec<Light>,
}

impl Tree {
    pub fn new(x: i32, y: i32) -> Box<dyn Animate> {
        let mut rng = rand::rng();
        let sprite = Sprite::new_at(&FRAME_1, &COLOURS, x, y);
        let lights: Vec<Light> = Sprite::new_at(&LIGHTS, &COLOURS, x, y)
            .step()
            .iter()
            .map(|point| Light {
                point: *point,
                colour: random_light_colour(&mut rng),
                rng: rand::rng(),
            })
            .collect();
        let mut group: Vec<Box<dyn Animate>> = Vec::with_capacity(lights.len() + 1);

        group.push(Box::new(sprite));
        for light in lights.iter() {
            group.push(Box::new(light.clone()));
        }

        Group::new(group)
    }
}
impl Animate for Light {
    fn step(&mut self) -> Points {
        if self.rng.random::<f32>() < 0.10 {
            self.colour = random_light_colour(&mut self.rng);
        }
        let c = LIGHT_COLOURS[self.colour];
        vec![Point { c, ..self.point }]
    }
    fn width(&self) -> usize {
        self.point.x as usize
    }
    fn height(&self) -> usize {
        self.point.y as usize
    }
}

impl Animate for Tree {
    fn step(&mut self) -> Points {
        self.sprite.step()
    }
    fn width(&self) -> usize {
        self.sprite.width()
    }
    fn height(&self) -> usize {
        self.sprite.height()
    }
}
