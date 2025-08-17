pub mod image;

use image::pixel::{BLUE, GREEN, RED, WHITE, YELLOW};
use image::{Image, Position};

fn main() {
    let mut img: Image = Image::blank(64, 64);

    let position_a: Position = Position { x: 7, y: 3 };
    let position_b: Position = Position { x: 12, y: 37 };
    let position_c: Position = Position { x: 62, y: 53 };

    img.line(RED, &position_a, &position_b);
    img.line(GREEN, &position_b, &position_c);
    img.line(BLUE, &position_c, &position_a);
    img.line(YELLOW, &position_a, &position_c);

    // draw vertices last, to overwrite pixel values.
    img.set(WHITE, &position_a);
    img.set(WHITE, &position_b);
    img.set(WHITE, &position_c);

    export(&img.ppm());
}

fn export(data: &Vec<u8>) {
    if let Err(e) = std::fs::write("output.ppm", data) {
        panic!("Error saving to file {}", e);
    }
}
