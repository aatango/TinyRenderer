pub mod geometry;
pub mod image;

use crate::geometry::Geometry;
use image::Image;
use image::pixel::Pixel;

const IMAGE_WIDTH: usize = 800;
const IMAGE_HEIGHT: usize = IMAGE_WIDTH;
const OBJ_FILE_PATH: &str = "obj/african_head/african_head.obj";

fn rand() -> u8 {
    let mut now = std::time::Instant::now().elapsed().as_nanos();
    now ^= now << 13;
    now ^= now >> 17;
    now ^= now << 5;

    now as u8
}

fn main() -> Result<(), std::io::Error> {
    let input_string = std::fs::read_to_string(OBJ_FILE_PATH).unwrap_or_default();
    let geometry = Geometry::decode_obj(input_string.as_str());

    let mut img: Image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    for face in geometry.faces {
        let colour = Pixel {
            red: rand(),
            green: rand(),
            blue: rand(),
        };

        img.triangle(
            colour,
            &geometry.vertices[face.0],
            &geometry.vertices[face.1],
            &geometry.vertices[face.2],
        );
    }

    std::fs::write("output.ppm", img.ppm())
}
