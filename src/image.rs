pub mod pixel;

use pixel::Pixel;

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
}

impl Image {
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

    pub fn triangle(&mut self, colour: Pixel, a: &Position, b: &Position, c: &Position) {
        let mut positions = [a, b, c];
        positions.sort_by_key(|p| p.y);
        positions.reverse();

        for y in positions[2].y..=positions[0].y {
            let left_x = find_x(y, positions[0], positions[2]);
            if y < positions[1].y {
                for x in left_x..=find_x(y, positions[2], positions[1]) {
                    self.set(colour, &Position { x, y });
                }
            } else {
                for x in left_x..=find_x(y, positions[1], positions[0]) {
                    self.set(colour, &Position { x, y });
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

fn find_x(y: usize, a: &Position, b: &Position) -> usize {
    let start: (f64, f64) = (a.x as f64, a.y as f64);
    let end: (f64, f64) = (b.x as f64, b.y as f64);

    let slope: f64 = (end.1 - start.1) / (end.0 - start.0);

    (((y as f64 - start.1) / slope) + start.0) as usize
}
