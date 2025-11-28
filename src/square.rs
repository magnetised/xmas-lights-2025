use crate::display::{Animate, Point, Points, Rgba, HEIGHT, WIDTH};

pub struct Square {
    points: Points,
}

impl Square {
    pub fn new(colour: Rgba, width: usize, height: usize) -> Box<Self> {
        let x_off = ((WIDTH - width) as f32 / 2.0).round() as usize;
        let y_off = ((HEIGHT - height) as f32 / 2.0).round() as usize;
        let mut points: Points = Vec::with_capacity(width * height);
        for x in x_off..(x_off + width) {
            for y in y_off..(y_off + height) {
                points.push(Point {
                    x: x,
                    y: y,
                    c: colour,
                })
            }
        }
        Box::new(Square { points: points })
    }
}

impl Animate for Square {
    fn step(&mut self) -> Points {
        return self.points.clone();
    }
}
