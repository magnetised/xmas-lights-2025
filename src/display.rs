use alpha_blend::rgba::F32x4Rgba;
// use alpha_blend::{BlendMode, RgbaBlend};
use alpha_blend::{BlendMode, RgbaBlend};
use array2d::Array2D;
use std::collections::HashMap;
use std::vec::IntoIter;

pub type Rgba = F32x4Rgba;
pub type Rgb = (u8, u8, u8);

#[derive(Clone, Copy, Debug)]
pub struct HSVa {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

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
    pub fn new(pixels: &[&str], colours: IntoIter<SpriteColour>) -> Self {
        let mut colour_lut = HashMap::new();
        for (s, colour) in colours {
            let ch = s.chars().nth(0).unwrap();
            colour_lut.insert(ch, colour);
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

pub fn rgb_to_hsv(color: Rgba) -> HSVa {
    // Normalize RGB values to 0.0 - 1.0
    // let r = color.r as f32 / 255.0;
    // let g = color.g as f32 / 255.0;
    // let b = color.b as f32 / 255.0;

    let max = color.r.max(color.g).max(color.b);
    let min = color.r.min(color.g).min(color.b);
    let delta = max - min;

    // Calculate Value
    let v = max;

    // Calculate Saturation
    let s = if max == 0.0 { 0.0f32 } else { delta / max };

    // Calculate Hue
    let h = if delta == 0.0 {
        0.0f32 // Undefined, achromatic (grey)
    } else if max == color.r {
        60.0 * (((color.g - color.b) / delta) % 6.0)
    } else if max == color.g {
        60.0 * (((color.b - color.r) / delta) + 2.0)
    } else {
        60.0 * (((color.r - color.g) / delta) + 4.0)
    };

    // Normalize hue to 0-360 range
    let h = if h < 0.0 { h + 360.0 } else { h };

    HSVa {
        h,
        s,
        v,
        a: color.a,
    }
}

pub fn hsv_to_rgb(hsv: HSVa) -> Rgba {
    let c = hsv.v * hsv.s;
    let h_prime = hsv.h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = hsv.v - c;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Rgba {
        r: (r + m),
        g: (g + m),
        b: (b + m),
        a: hsv.a,
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
