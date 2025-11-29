use std::time::Duration;

use std::{panic, process, thread};

mod bounce;
mod display;
mod leds;
mod snow;
mod square;
// mod null;
mod terminal;

use display::{Animate, Display, Layer, Points, rgba};

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
        snow::Snow::new(0),
        square::Square::new(rgba(1.0, 1.0, 0.0, 1.0), 5, 4),
        snow::Snow::new(2),
        snow::Snow::new(3),
        snow::Snow::new(4),
        // bounce::Bounce::random(rgba(1.0, 0.4, 0.0, 1.0)),
        // bounce::Bounce::completely_random(),
        // bounce::Bounce::random(rgba(0.0, 1.0, 0.0, 1.0)),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
        // bounce::Bounce::random_a(0.0, 1.0, 1.0),
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
                base.blend(point.x, point.y, point.c)
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
