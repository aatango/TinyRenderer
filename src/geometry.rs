#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, PartialEq)]
pub struct Triangle(pub usize, pub usize, pub usize);

#[derive(Debug, PartialEq)]
pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Triangle>,
}

impl Geometry {
    #[allow(clippy::just_underscores_and_digits)]
    pub fn decode_obj(input: &str) -> Self {
        let parse_vertex_line = |line: &str| -> Vertex {
            let [x, y, z]: [f64; 3] = line
                .split_whitespace()
                .skip(1) // Skip vertex marker 'f'
                .map(|token| token.parse::<f64>().expect("Cannot convert into `f64`"))
                .collect::<Vec<f64>>()
                .try_into()
                .expect("Expected only three tokens");

            Vertex { x, y, z }
        };

        let parse_face_line = |line: &str| -> Triangle {
            let [_0, _1, _2]: [usize; 3] = line
                .split_whitespace()
                .skip(1) // Skip face marker 'f'
                .map(|elem| elem.split("/").next().unwrap())
                // Subtract converted token, since OBJ faces index the vertices starting at 1
                .map(|token| token.parse::<usize>().expect("Cannot convert into `usize`") - 1)
                .collect::<Vec<usize>>()
                .try_into()
                .expect("Expected only three tokens");

            Triangle(_0, _1, _2)
        };

        let vertices: Vec<Vertex> = input
            .lines()
            .filter(|line| line.starts_with("v "))
            .map(parse_vertex_line)
            .collect();

        let faces: Vec<Triangle> = input
            .lines()
            .filter(|line| line.starts_with("f "))
            .map(parse_face_line)
            .collect();

        Geometry { vertices, faces }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_obj_file() {
        let input: &str = "v -1 -1 -1\nv  1 -1 -1\nv  1 -1  1\nv -1 -1  1\n\nvt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\nvn 0 1 0\n\nf 3/3/1 2/2/1 1/1/1\nf 4/4/1 3/3/1 1/1/1";

        let expected = Geometry {
            vertices: vec![
                Vertex {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                },
                Vertex {
                    x: 1.0,
                    y: -1.0,
                    z: -1.0,
                },
                Vertex {
                    x: 1.0,
                    y: -1.0,
                    z: 1.0,
                },
                Vertex {
                    x: -1.0,
                    y: -1.0,
                    z: 1.0,
                },
            ],
            faces: vec![Triangle(2, 1, 0), Triangle(3, 2, 0)],
        };

        assert_eq!(
            Geometry::decode_obj(input),
            expected,
            "Cannot decode '.obj' file"
        );
    }
}
