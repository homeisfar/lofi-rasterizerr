#[derive(Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub l: u8
}

pub struct ImageDetails {
    pub width: u32,
    pub height: u32
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8, l: u8) -> Pixel {
        Pixel {
            r, g, b, a, l
        }
    }
    
    pub fn b_or_w(&self) -> Pixel {
        if self.l > 127 {
            Pixel::new(255, 255, 255, 255, 255)
        } else {
            Pixel::new(0, 0, 0, 255, 0)
        }
    }

    pub fn to_luma(r: u8, g: u8, b: u8) -> u8 {
        ((r as f32) * 0.212) as u8 +
        ((g as f32) * 0.715) as u8 +
        ((b as f32) * 0.072) as u8
    }
}
