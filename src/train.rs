use crate::display::{
    hsv_to_rgb, Animate, HSVa, Point, Points, Rgba, Sprite, SpriteColour, HEIGHT, WIDTH,
};

const PERIOD: usize = 12;

pub trait PartWidth {
    fn width(&self) -> usize;
}

pub trait TrainPart: Animate + PartWidth {}

pub struct Train {
    parts: Vec<Part>,
    x: i32,
    y: i32,

    v: i32,
    n: usize,
    w: usize,
}

pub struct Part {
    part: Box<dyn TrainPart>,
    x: i32,
    y: i32,
}

pub fn board(part: Box<dyn TrainPart>, x: i32, y: i32) -> Part {
    Part { part, x, y }
}

impl Train {
    pub fn new(parts: Vec<Part>, y: i32) -> Box<Self> {
        let w: usize = parts.iter().fold(0, |sum, part| sum + part.width());
        Box::new(Self {
            parts,
            x: 10,
            y: y,
            v: -1,
            n: 0,
            w,
        })
    }
}

impl Animate for Part {
    fn step(&mut self) -> Points {
        self.part
            .step()
            .iter_mut()
            .map(|p| Point {
                x: p.x + self.x,
                y: p.y + self.y,
                ..*p
            })
            .collect()
    }
}
impl Part {
    fn width(&self) -> usize {
        self.part.width()
    }
}
impl Animate for Train {
    fn step(&mut self) -> Points {
        self.n = (self.n + 1) % PERIOD;
        if self.n == 0 {
            self.x += self.v;
        }
        let mut points: Vec<Point> = self.parts.iter_mut().flat_map(|part| part.step()).collect();
        points.iter_mut().for_each(|p| {
            if self.v > 0 {
                p.x = self.w as i32 - p.x;
            }
            p.x = p.x + self.x;
            p.y = p.y + self.y;
            // going right - reversed
        });
        println!("{} {} {}", self.x, self.v, self.w);
        if (self.v < 0 && self.x < -(self.w as i32)) || (self.v > 0 && self.x > WIDTH as i32) {
            self.v = -self.v;
            println!("FLIP {}", self.v);
        }
        points
    }
}
