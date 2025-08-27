pub mod pixel;

use pixel::Pixel;

use crate::geometry::Vertex;

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn is_inside_triangle(&self, a: &Position, b: &Position, c: &Position) -> bool {
        let total_area: f64 = triangle_area(a, b, c);

        let alpha: f64 = triangle_area(self, a, b) / total_area;
        let beta: f64 = triangle_area(self, b, c) / total_area;
        let gamma: f64 = triangle_area(self, c, a) / total_area;

        alpha.is_sign_positive() && beta.is_sign_positive() && gamma.is_sign_positive()
    }
}

fn triangle_area(a: &Position, b: &Position, c: &Position) -> f64 {
    let (ax, ay) = (a.x as f64, a.y as f64);
    let (bx, by) = (b.x as f64, b.y as f64);
    let (cx, cy) = (c.x as f64, c.y as f64);

    (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)) / 2.0
}

fn bounding_box(positions: Vec<&Position>) -> (Position, Position) {
    let min = Position {
        x: positions.iter().map(|p| p.x).min().unwrap(),
        y: positions.iter().map(|p| p.y).min().unwrap(),
    };

    let max = Position {
        x: positions.iter().map(|p| p.x).max().unwrap(),
        y: positions.iter().map(|p| p.y).max().unwrap(),
    };

    (min, max)
}

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
}

impl Image {
    fn project_vertex(&self, v: &Vertex) -> Position {
        Position {
            x: ((v.x + 1.0) * self.width as f64) as usize / 2,
            y: ((v.y + 1.0) * self.height as f64) as usize / 2,
        }
    }

    pub fn blank(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![
                Pixel {
                    red: 0,
                    green: 0,
                    blue: 0
                };
                width * height
            ],
        }
    }

    pub fn set(&mut self, pixel: Pixel, position: &Position) {
        if position.x < self.width && position.y < self.height {
            self.data[position.x + self.width * (self.width - position.y - 1)] = pixel;
        }
    }

    pub fn line(&mut self, colour: Pixel, start: &Position, end: &Position) {
        let diff_x: i64 = end.x as i64 - start.x as i64;
        let diff_y: i64 = end.y as i64 - start.y as i64;

        if diff_y.abs() < diff_x.abs() {
            if start.x > end.x {
                self.line_low(colour, end, start);
            } else {
                self.line_low(colour, start, end);
            }
        } else if start.y > end.y {
            self.line_high(colour, end, start);
        } else {
            self.line_high(colour, start, end);
        }
    }

    fn line_low(&mut self, colour: Pixel, start: &Position, end: &Position) {
        let dx: i64 = end.x as i64 - start.x as i64;

        let mut dy: i64 = end.y as i64 - start.y as i64;
        let mut yi: i64 = 1;
        let mut y: i64 = start.y as i64;

        if dy < 0 {
            yi = -1;
            dy = -dy;
        }

        let mut dd: i64 = (2 * dy) - dx;

        for x in start.x..=end.x {
            self.set(colour, &Position { x, y: y as usize });
            if dd > 0 {
                y += yi;
                dd += 2 * (dy - dx);
            } else {
                dd += 2 * dy;
            }
        }
    }

    fn line_high(&mut self, colour: Pixel, start: &Position, end: &Position) {
        let mut dx: i64 = end.x as i64 - start.x as i64;
        let mut xi: i64 = 1;

        let dy: i64 = end.y as i64 - start.y as i64;

        if dx < 0 {
            xi = -1;
            dx = -dx;
        }

        let mut dd: i64 = (2 * dx) - dy;
        let mut x: i64 = start.x as i64;

        for y in start.y..=end.y {
            self.set(colour, &Position { x: x as usize, y });
            if dd > 0 {
                x += xi;
                dd += 2 * (dx - dy);
            } else {
                dd += 2 * dx;
            }
        }
    }

    pub fn triangle(&mut self, colour: Pixel, i: &Vertex, j: &Vertex, k: &Vertex) {
        let a: &Position = &self.project_vertex(i);
        let b: &Position = &self.project_vertex(j);
        let c: &Position = &self.project_vertex(k);

        if triangle_area(a, b, c) < 0.0 {
            return;
        }

        let (bottom_left, top_right): (Position, Position) = bounding_box(vec![&a, &b, &c]);

        for x in bottom_left.x..=top_right.x {
            for y in bottom_left.y..=top_right.y {
                let px = Position { x, y };
                if px.is_inside_triangle(a, b, c) {
                    self.set(colour, &px);
                }
            }
        }
    }

    pub fn ppm(&self) -> Vec<u8> {
        let mut ppm = Vec::new();

        ppm.extend_from_slice(b"P6\n");
        ppm.extend_from_slice(format!("{} {}\n", self.width, self.height).as_bytes());
        ppm.extend_from_slice(b"255\n");

        for pixel in &self.data {
            ppm.push(pixel.red);
            ppm.push(pixel.green);
            ppm.push(pixel.blue);
        }

        ppm
    }
}
