use alpha_blend::rgba::F32x4Rgba;
// use alpha_blend::{BlendMode, RgbaBlend};
use alpha_blend::{BlendMode, RgbaBlend};
use array2d::Array2D;

pub type Rgba = F32x4Rgba;
pub type Rgb = (u8, u8, u8);

pub const BLACK: Rgb = (0, 0, 0);

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub c: Rgba,
}

pub type Points = Vec<Point>;

const CLEAR: F32x4Rgba = F32x4Rgba {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};
pub const WIDTH: usize = 15;
pub const HEIGHT: usize = 20;

pub trait Display {
    fn render(&mut self, grid: &Array2D<Rgb>);
}

pub trait Animate {
    fn step(&mut self) -> Vec<Point>;
}

#[derive(Clone, Debug)]
pub struct Layer {
    grid: Array2D<Rgba>,
    pub opacity: f32,
    pub width: usize,
    pub height: usize,
}
pub fn array<T>(element: T) -> Array2D<T>
where
    T: Clone,
{
    return Array2D::filled_with(element, HEIGHT, WIDTH);
}
pub fn rgba_to_rgb(rgba: Rgba) -> Rgb {
    (
        (rgba.r * 255.0).round() as u8,
        (rgba.g * 255.0).round() as u8,
        (rgba.b * 255.0).round() as u8,
    )
}
impl Layer {
    pub fn new(opacity: f32) -> Self {
        Layer {
            opacity: opacity,
            grid: array(CLEAR),
            width: WIDTH,
            height: HEIGHT,
        }
    }

    pub fn filled_with(colour: Rgba) -> Self {
        Layer {
            opacity: 1.0,
            grid: array(colour),
            width: WIDTH,
            height: HEIGHT,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, colour: Rgba) {
        let blended = Rgba {
            a: colour.a * self.opacity,
            ..colour
        };
        self.grid.set(y, x, blended);
    }

    pub fn blend(&mut self, x: usize, y: usize, colour: Rgba) {
        let dst = self.get(x, y);
        let blended = BlendMode::SourceOver.apply(colour, dst);
        self.set(x, y, blended);
    }

    pub fn get(&self, x: usize, y: usize) -> Rgba {
        return *self.grid.get(y, x).unwrap();
    }

    pub fn clear(&mut self) {
        self.grid = array(CLEAR);
    }
}
