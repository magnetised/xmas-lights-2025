use crate::display::{Animate, Layer, Rgba};

pub struct Square {
    layer: Layer,
}

impl Square {
    pub fn new(colour: Rgba, width: usize, height: usize, opacity: f32) -> Self {
        let mut layer = Layer::new(opacity);

        let x_off = ((layer.width - width) as f32 / 2.0).round() as usize;
        let y_off = ((layer.height - height) as f32 / 2.0).round() as usize;
        for x in x_off..(x_off + width) {
            for y in y_off..(y_off + height) {
                println!("x: {}; y: {}", x, y);
                layer.set(x, y, colour);
            }
        }
        Square {
            layer: layer.clone(),
        }
    }
}

impl Animate for Square {
    fn step(&mut self) -> Layer {
        return self.layer.clone();
    }
}
