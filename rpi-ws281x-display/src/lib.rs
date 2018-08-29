extern crate rand;
extern crate cgmath;

pub mod animations;

pub trait Animation<T: PixelDisplay> {
    fn setup(&mut self, display: &mut T);
    fn update(&mut self, display: &mut T, delta: f64, elapsed: f64);
    fn is_finished(&self, display: &mut T, elapsed: f64) -> bool;
}

pub trait PixelDisplay {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn set_at<T: Into<usize>>(&mut self, x: T, y: T, pixel: Pixel);
    fn get_at<T: Into<usize>>(&self, x: T, y: T) -> Pixel;
    fn render(&mut self);
    fn clear(&mut self);
}

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
        Pixel(w.into() << 24 | r.into() << 16 | g.into() << 8 | b.into())
    }

    pub fn w(self) -> u8 {
        ((u32::from(self) & 0xff000000) >> 24) as u8
    }

    pub fn r(self) -> u8 {
        ((u32::from(self) & 0x00ff0000) >> 16) as u8
    }

    pub fn g(self) -> u8 {
        ((u32::from(self) & 0x0000ff00) >> 8) as u8
    }

    pub fn b(self) -> u8 {
        (u32::from(self) & 0x0000ff) as u8
    }

    pub fn at_brightness(&self, brightness: u8) -> Pixel {
        let scalar = brightness as f64 / 255.0;
        self.scale(scalar)
    }

    pub fn scale(&self, scalar: f64) -> Pixel {
        Pixel::new(
            (self.r() as f64 * scalar) as u8,
            (self.g() as f64 * scalar) as u8,
            (self.b() as f64 * scalar) as u8,
            (self.w() as f64 * scalar) as u8,
        )
    }
}
