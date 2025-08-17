#[derive(Copy, Clone)]
pub struct Pixel {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

pub const RED: Pixel = Pixel {
    red: 255,
    green: 0,
    blue: 0,
};

pub const GREEN: Pixel = Pixel {
    red: 0,
    green: 255,
    blue: 0,
};

pub const BLUE: Pixel = Pixel {
    red: 0,
    green: 0,
    blue: 255,
};

pub const YELLOW: Pixel = Pixel {
    red: 255,
    green: 255,
    blue: 0,
};

pub const WHITE: Pixel = Pixel {
    red: 255,
    green: 255,
    blue: 255,
};
