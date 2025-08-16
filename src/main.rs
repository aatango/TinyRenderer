fn main() {
    let mut img: Image = Image::blank(64, 64);

    img.set(RED, Position { x: 52, y: 41 });

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

fn export(data: &Vec<u8>) {
    if let Err(e) = std::fs::write("output.ppm", data) {
        panic!("Error saving to file {}", e);
    }
}
