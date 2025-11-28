use crate::display::{Animate, Layer, Rgba};

pub struct Bounce {
    layer: Layer,
    colour: Rgba,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Bounce {
    pub fn new(colour: Rgba, vx: f32, vy: f32, opacity: f32) -> Self {
        Bounce {
            layer: Layer::new(opacity),
            colour: colour,
            x: 0.0,
            y: 0.0,
            vx: vx,
            vy: vy,
        }
    }
}

impl Animate for Bounce {
    fn step(&mut self) -> Layer {
        self.x += self.vx;
        self.y += self.vy;

        if self.x > self.layer.width as f32 {
            self.x = self.layer.width as f32;
            self.vx = -self.vx;
        }
        if self.y > self.layer.height as f32 {
            self.y = self.layer.height as f32;
            self.vy = -self.vy;
        }
        if self.x < 0.0 {
            self.x = 0.0;
            self.vx = -self.vx;
        }
        if self.y < 0.0 {
            self.y = 0.0;
            self.vy = -self.vy;
        }
        self.layer.clear();

        self.layer.set(
            self.x.round() as usize,
            self.y.round() as usize,
            self.colour,
        );

        return self.layer.clone();
    }
}
