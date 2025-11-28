#![allow(dead_code)]

use array2d::Array2D;

use crate::display::{Display, Rgb, HEIGHT, WIDTH};

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Self {
        Terminal {}
    }
}

impl Display for Terminal {
    fn render(&mut self, grid: &Array2D<Rgb>) {
        const CHARACTER: &str = "█";

        print!("\x1B[2J\x1B[1;1H");

        for y in 0..HEIGHT {
            let mut row: Vec<String> = Vec::with_capacity(WIDTH);
            for x in 0..WIDTH {
                // println!("x: {}; y: {}", x, y);

                let (r, g, b) = grid.get(y, x).unwrap();
                // println!("r: {}; g: {}; b: {}", r, g, b);
                let colour = format!("{};{};{}", r, g, b);
                row.push(format!("\x1B[38;2;{0}m{1}\x1B[0m", colour, CHARACTER));
            }
            print!("{}\n", row.join(""));
        }

        // let mut lights: Vec<String> = Vec::with_capacity(bins.len());
        //
        // for (i, &magnitude) in bins.iter().enumerate() {
        //     let key_colour = key_colour(i + 1);
        //     if magnitude > peak_magnitudes[i] {
        //         peak_magnitudes[i] = magnitude;
        //     } else {
        //         peak_magnitudes[i] *= config.fade;
        //     }
        //
        //     let brightness = peak_magnitudes[i];
        //     // let brightness = 255.0;
        //     // let character = "●";
        //     let character = "█";
        //     // let character = "■";
        //
        //     let colour = match key_colour {
        //         KeyColour::Black => {
        //             let (r, g, b) = config.black_colour(brightness);
        //             format!("{};{};{}", r, g, b)
        //         }
        //         KeyColour::White => {
        //             let (r, g, b) = config.white_colour(brightness);
        //             format!("{};{};{}", r, g, b)
        //         }
        //     };
        //     lights.push(format!(
        //         // "\x1B[38;2;{0};{0};0m{1}\x1B[0m",
        //         // "\x1B[38;2;{0};{0};0m{1}\x1B[0m",
        //         "\x1B[38;2;{0}m{1}\x1B[0m",
        //         colour, character
        //     ));
        // }
        // // lights.join(""),
        // print!("\x1B[2J\x1B[1;1H{}\n{}", lights.join(""), lights.join(""),);
        // // print!("{}\n", lights.join(""));
    }
}
