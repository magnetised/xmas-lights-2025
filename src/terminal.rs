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

        let line = " ".repeat(WIDTH);

        print!("\x1B[?25l");
        print!("\x1B[2J\x1B[1;1H");

        println!("");
        println!("");

        for y in 0..HEIGHT {
            let mut row: Vec<String> = Vec::with_capacity(WIDTH);
            row.push("    ".to_string());
            for x in 0..WIDTH {
                // println!("x: {}; y: {}", x, y);

                let (r, g, b) = grid.get(y, x).unwrap();
                // println!("r: {}; g: {}; b: {}", r, g, b);
                let colour = format!("{};{};{}", r, g, b);
                row.push(format!(
                    // "\x1B[38;2;{0}m{1}{1}{1}{1}{1}{1}\x1B[0m",
                    "\x1B[38;2;{0}m{1}     \x1B[0m",
                    colour, CHARACTER
                ));
            }
            print!("{}\n{}\n{}\n", line, row.join(""), line);
        }
    }
}
