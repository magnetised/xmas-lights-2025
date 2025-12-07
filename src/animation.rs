use crate::display::{
    Animate, Points,
};
use rand::prelude::*;

pub struct Animation {
    frames: Vec<Box<dyn Animate>>,
    f: usize,
    period: usize,
    n: usize,
    w: usize,
    h: usize,
}

impl Animation {
    pub fn new(frames: Vec<Box<dyn Animate>>, period: usize) -> Box<Self> {
        let w: i32 = frames.iter().fold(std::i32::MIN, |acc, animate| {
            acc.max(animate.width() as i32)
        });
        let h: i32 = frames.iter().fold(std::i32::MIN, |acc, animate| {
            acc.max(animate.height() as i32)
        });

        let mut rng = rand::rng();
        Box::new(Self {
            frames: frames,
            f: 0,
            period: period,
            n: rng.random_range(0..period as usize),
            w: w as usize,
            h: h as usize,
        })
    }
}
impl Animate for Animation {
    fn step(&mut self) -> Points {
        self.n = (self.n + 1) % self.period;
        if self.n == 0 {
            self.f = (self.f + 1) % self.frames.len();
        }
        let frame = self.frames.get_mut(self.f).unwrap();
        frame.step()
    }
    fn width(&self) -> usize {
        self.w
    }
    fn height(&self) -> usize {
        self.h
    }
}
