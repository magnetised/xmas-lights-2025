// use std::io::{self, BufRead, BufReader};
use std::time::Duration;

use std::{panic, process, thread};

use array2d::Array2D;

mod bounce;
mod display;
mod layer;
mod square;
// mod leds;
// mod null;
mod terminal;

use display::{Animate, Display, Layer, Points, Rgba};

const SAMPLE_SIZE: usize = 2usize.pow(13);
const RINGBUFFER_SIZE: usize = SAMPLE_SIZE;

enum Ping {
    Audio,
    Timeout,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    let mut grid = display::array(display::BLACK);

    // first is at top, so over everything else
    let mut layers: Vec<Box<dyn Animate>> = vec![
        Box::new(bounce::Bounce::new(
            Rgba {
                r: 1.0,
                g: 0.4,
                b: 0.0,
                a: 1.0,
            },
            0.023,
            0.17,
        )),
        Box::new(square::Square::new(
            Rgba {
                r: 1.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            5,
            4,
        )),
        Box::new(bounce::Bounce::new(
            Rgba {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 0.3,
            },
            0.06,
            0.2,
        )),
        Box::new(bounce::Bounce::new(
            Rgba {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            0.80,
            0.11,
        )),
    ];

    layers.reverse();

    let mut display = display_impl();

    loop {
        let animated: Vec<Points> = layers
            .iter_mut()
            .map(|animation| animation.step())
            .collect();

        let mut base = Layer::filled_with(Rgba {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        });

        for layer in animated.iter() {
            for point in layer {
                base.blend(point.x, point.y, point.c)
            }
        }
        for y in 0..display::HEIGHT {
            for x in 0..display::WIDTH {
                let blended = base.get(x, y);
                let rgb = display::rgba_to_rgb(blended);
                grid.set(y, x, rgb);
            }
        }
        display.render(&grid);

        thread::sleep(Duration::from_millis(16));
    }
}

#[cfg(feature = "leds")]
fn display_impl() -> impl display::Display {
    leds::LEDs::new()
}

#[cfg(feature = "terminal")]
fn display_impl() -> impl display::Display {
    terminal::Terminal::new()
}

#[cfg(feature = "no-display")]
fn display_impl() -> impl display::Display {
    null::Null::new()
}
