use std::time::Duration;

use std::{panic, process, thread};

mod bounce;
mod display;
mod leds;
mod present;
mod reindeer;
mod seventeen;
mod snow;
mod snowflake;
mod square;
mod terminal;

use display::{rgba, Animate, Display, Layer, Points};

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
        snow::Snow::new(1, 5),
        reindeer::Reindeer::new(6, 10),
        snowflake::Snowflake::new(),
        snow::Snow::new(2, 6),
        // present::Present::large(1, 15, rgba(0.8, 0.0, 0.0, 1.0), rgba(0.8, 0.8, 0.0, 1.0)),
        // present::Present::small(3, 12, rgba(0.0, 0.8, 0.0, 1.0), rgba(0.8, 0.8, 0.8, 1.0)),
        // present::Present::small(6, 17, rgba(0.8, 0.5, 0.3, 1.0), rgba(0.8, 0.0, 0.8, 1.0)),
        // square::Square::new(6, 14),
        seventeen::Seventeen::small(1, 1),
        // seventeen::Seventeen::large(2, 3),
        snow::Snow::new(3, 8),
        snow::Snow::new(4, 20),
    ];

    layers.reverse();

    let mut display = display_impl();

    loop {
        let animated: Vec<Points> = layers
            .iter_mut()
            .map(|animation| animation.step())
            .collect();

        let mut base = Layer::filled_with(rgba(0.0, 0.0, 0.0, 1.0));

        for layer in animated.iter() {
            for point in layer {
                if point.x >= 0
                    && point.x < display::WIDTH as i32
                    && point.y >= 0
                    && point.y < display::HEIGHT as i32
                {
                    base.blend(point.x as usize, point.y as usize, point.c)
                }
            }
        }
        for y in 0..display::HEIGHT {
            for x in 0..display::WIDTH {
                let blended = base.get(x, y);
                let rgb = display::rgba_to_rgb(blended);
                _ = grid.set(y, x, rgb);
            }
        }
        display.render(&grid);

        thread::sleep(Duration::from_millis(32));
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

// #[cfg(feature = "no-display")]
// fn display_impl() -> impl display::Display {
//     null::Null::new()
// }
