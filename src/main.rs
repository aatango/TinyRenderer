pub mod geometry;
pub mod image;

use image::pixel::{BLUE, GREEN, RED};
use image::{Image, Position};

const IMAGE_WIDTH: usize = 128;
const IMAGE_HEIGHT: usize = IMAGE_WIDTH;

fn main() -> Result<(), std::io::Error> {
    let mut img: Image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    img.triangle(
        RED,
        &Position { x: 7, y: 45 },
        &Position { x: 35, y: 100 },
        &Position { x: 45, y: 60 },
    );

    img.triangle(
        GREEN,
        &Position { x: 120, y: 35 },
        &Position { x: 90, y: 5 },
        &Position { x: 45, y: 120 },
    );

    img.triangle(
        BLUE,
        &Position { x: 115, y: 83 },
        &Position { x: 90, y: 80 },
        &Position { x: 85, y: 120 },
    );

    std::fs::write("output.ppm", img.ppm())
}
