use rpi_ws281x_display::{PixelDisplay, Pixel};

pub struct Display([Pixel; 49]);

impl Default for Display {
    fn default() -> Self {
        Display([Pixel::default(); 49])
    }
}

impl PixelDisplay for Display {
    fn rows(&self) -> usize {
        7
    }
    fn cols(&self) -> usize {
        7
    }
    
    fn set_at<T: Into<usize>>(&mut self, x: T, y: T, pixel: Pixel) {
        let (x, y) = (x.into(), y.into());
        self.0[x + y * 7] = pixel;
    }
    
    fn get_at<T: Into<usize>>(&self, x: T, y: T) -> Pixel {
        let (x, y) = (x.into(), y.into());
        self.0[x + y * 7]
    }
    
    fn render(&mut self) {
        ()
    }
    
    fn clear(&mut self) {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                self.set_at(col, row, Pixel::new(0u8, 0u8, 0u8, 0u8));
            }
        }
    }
}
