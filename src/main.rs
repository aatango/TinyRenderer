pub mod geometry;
pub mod image;

use crate::geometry::{Geometry, Vertex};
use image::pixel::{RED, WHITE};
use image::{Image, Position};

const IMAGE_WIDTH: usize = 800;
const IMAGE_HEIGHT: usize = IMAGE_WIDTH;
const OBJ_FILE_PATH: &str = "obj/african_head/african_head.obj";

fn main() -> Result<(), std::io::Error> {
    let input_string = std::fs::read_to_string(OBJ_FILE_PATH).unwrap_or_default();
    let geometry = Geometry::decode_obj(input_string.as_str());

    let mut img: Image = Image::blank(IMAGE_WIDTH, IMAGE_HEIGHT);

    let projected_vertices: Vec<Position> = geometry
        .vertices
        .into_iter()
        .map(|v: Vertex| Position {
            x: ((v.x + 1.0) * IMAGE_WIDTH as f64) as usize / 2,
            y: ((v.y + 1.0) * IMAGE_HEIGHT as f64) as usize / 2,
        })
        .collect();

    for face in geometry.faces {
        img.line(
            RED,
            &projected_vertices[face.0],
            &projected_vertices[face.1],
        );
        img.line(
            RED,
            &projected_vertices[face.1],
            &projected_vertices[face.2],
        );
        img.line(
            RED,
            &projected_vertices[face.2],
            &projected_vertices[face.0],
        );
    }

    // draw vertices last, to overwrite pixel values.
    projected_vertices.iter().for_each(|p| img.set(WHITE, p));

    std::fs::write("output.ppm", img.ppm())
}
