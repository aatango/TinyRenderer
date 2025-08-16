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

#[derive(Copy, Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

struct Position {
    x: usize,
    y: usize,
}

struct Image {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
}

impl Image {
    fn blank(width: usize, height: usize) -> Self {
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

    fn set(&mut self, pixel: Pixel, position: &Position) {
        if position.x >= self.width || position.y >= self.height {
            panic!("Setting a pixel outside the image!");
        }

        self.data[position.x + self.width * position.y] = pixel;
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

    fn line(&mut self, colour: Pixel, start: &Position, end: &Position) {
        let diff_x: i64 = end.x as i64 - start.x as i64;
        let diff_y: i64 = end.y as i64 - start.y as i64;

        if diff_y.abs() < diff_x.abs() {
            if start.x > end.x {
                self.line_low(colour, end, start);
            } else {
                self.line_low(colour, start, end);
            }
        } else {
            if start.y > end.y {
                self.line_high(colour, end, start);
            } else {
                self.line_high(colour, start, end);
            }
        }
    }

    fn ppm(&self) -> Vec<u8> {
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

const RED: Pixel = Pixel {
    red: 255,
    green: 0,
    blue: 0,
};

const GREEN: Pixel = Pixel {
    red: 0,
    green: 255,
    blue: 0,
};

const BLUE: Pixel = Pixel {
    red: 0,
    green: 0,
    blue: 255,
};

const YELLOW: Pixel = Pixel {
    red: 255,
    green: 255,
    blue: 0,
};

const WHITE: Pixel = Pixel {
    red: 255,
    green: 255,
    blue: 255,
};

fn export(data: &Vec<u8>) {
    if let Err(e) = std::fs::write("output.ppm", data) {
        panic!("Error saving to file {}", e);
    }
}
