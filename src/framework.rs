#[derive(Debug, Copy, Clone)]
pub struct Pixel(u32);

impl Default for Pixel {
    fn default() -> Self {
        Self::new(0u32, 0u32, 0u32, 0u32)
    }
}

impl From<Pixel> for u32 {
    fn from(pixel: Pixel) -> Self {
        pixel.0
    }
}

impl From<u32> for Pixel {
    fn from(val: u32) -> Self {
        Pixel(val)
    }
}

impl Pixel {
    pub fn new<T: Into<u32>>(r: T, g: T, b: T, w: T) -> Pixel {
        Pixel(r.into() << 24 | g.into() << 16 | b.into() << 8 | w.into())
    }

    pub fn r(self) -> u8 {
        ((u32::from(self) & 0xff000000) >> 24) as u8
    }

    pub fn g(self) -> u8 {
        ((u32::from(self) & 0x00ff0000) >> 16) as u8
    }

    pub fn b(self) -> u8 {
        ((u32::from(self) & 0x0000ff00) >> 8) as u8
    }

    pub fn w(self) -> u8 {
        (u32::from(self) & 0x0000ff) as u8
    }
}

pub struct Display([Pixel; 49]);

impl Default for Display {
    fn default() -> Self {
        Display([Pixel::default(); 49])
    }
}

impl Display {
    pub fn set_at<T: Into<usize>>(&mut self, x: T, y: T, pixel: Pixel) {
        let (x, y) = (x.into(), y.into());
        if x < 7 && y < 7 {
            self.0[x + y * 7] = pixel;
        }
    }

    pub fn get_at<T: Into<usize>>(&self, x: T, y: T) -> Option<Pixel> {
        let (x, y) = (x.into(), y.into());
        if x < 7 && y < 7 {
            Some(self.0[x + y * 7])
        } else {
            None
        }
    }
}
