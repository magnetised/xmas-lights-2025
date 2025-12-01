use alpha_blend::rgba::F32x4Rgba;
// use alpha_blend::{BlendMode, RgbaBlend};
use alpha_blend::{BlendMode, RgbaBlend};
use array2d::Array2D;
use std::collections::HashMap;

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
pub type SpriteColour<'a> = (&'a str, Rgba);

pub struct SpriteConfig<'a> {
    pub pixels: Vec<&'a str>,
    pub colours: Vec<SpriteColour<'a>>,
}

pub struct Sprite {
    points: Vec<Point>,
    pub x: usize,
    pub y: usize,
}

const SPACE: char = ' ';

impl Sprite {
    pub fn new(pixels: &[&str], colours: &[SpriteColour]) -> Self {
        let mut colour_lut = HashMap::new();
        for (s, colour) in colours.iter() {
            let ch = s.chars().nth(0).unwrap();
            colour_lut.insert(ch, *colour);
        }
        let mut points: Vec<Point> = Vec::new();
        for (y, l) in pixels.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c != SPACE {
                    points.push(Point {
                        x,
                        y,
                        c: *colour_lut.get(&c).unwrap(),
                    })
                }
            }
        }
        return Sprite {
            points: points,
            x: 0,
            y: 0,
        };
    }

    pub fn position(&mut self, x: usize, y: usize) -> &Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn render(&self) -> Points {
        self.points
            .iter()
            .map(|p| Point {
                x: p.x + self.x,
                y: p.y + self.y,
                c: p.c,
            })
            .collect()
    }
    pub fn render_at(&self, x: usize, y: usize) -> Points {
        self.points
            .iter()
            .map(|p| Point {
                x: p.x + x,
                y: p.y + y,
                c: p.c,
            })
            .collect()
    }
}

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
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Rgba {
    Rgba {
        r: r,
        g: g,
        b: b,
        a: a,
    }
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
        self.grid.set(y, x, blended).unwrap();
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
