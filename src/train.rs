use crate::display::{Animate, Point, Points, WIDTH};
use rand::prelude::*;

const PERIOD: usize = 6;

pub struct Train {
    parts: Vec<Part>,
    x: i32,
    y: i32,
    min_y: i32,
    max_y: i32,

    v: i32,
    n: usize,
    w: usize,
    h: usize,
    rng: ThreadRng,
}

pub struct Part {
    part: Box<dyn Animate>,
    x: i32,
    y: i32,
}

pub fn board(part: Box<dyn Animate>, x: i32, y: i32) -> Part {
    Part { part, x, y }
}

impl Train {
    pub fn new(parts: Vec<Part>, min_y: i32, max_y: i32) -> Box<Self> {
        // let w: usize = parts.iter().fold(0, |sum, part| sum + part.width());
        let last = parts.last().unwrap();
        let w = last.x as usize + last.part.width();
        let h: usize = parts.iter().fold(0, |sum, part| sum + part.height());
        Box::new(Self {
            parts,
            x: 10,
            y: min_y,
            min_y,
            max_y,
            v: -1,
            n: 0,
            w,
            h,
            rng: rand::rng(),
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
    fn width(&self) -> usize {
        self.part.width()
    }
    fn height(&self) -> usize {
        self.part.height()
    }
}
impl Part {
    fn width(&self) -> usize {
        self.part.width()
    }
}
impl Animate for Train {
    fn step(&mut self) -> Points {
        let gap: i32 = 12;
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
        });
        if (self.v < 0 && self.x < -(self.w as i32 + gap))
            || (self.v > 0 && self.x > (WIDTH as i32 + gap))
        {
            self.y = self.rng.random_range(self.min_y..=self.max_y);
            self.v = -self.v;
        }
        points
    }
    fn width(&self) -> usize {
        self.w
    }
    fn height(&self) -> usize {
        self.h
    }
}
