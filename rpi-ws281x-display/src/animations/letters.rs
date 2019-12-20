use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{FontCollection, Font, Scale};
use std::env;
use std::path::Path;

use ::{PixelDisplay, Animation};
use Pixel;

static ALPHA: [char; 26] = [
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z',
];

#[derive(Default)]
pub struct Letters {
    color: Pixel,
}

impl<T: PixelDisplay> Animation<T> for Letters {
    fn setup(&mut self, _display: &mut T) {
        self.color = Pixel::new(255u8, 255u8, 255u8, 255u8);
    }

    fn update(&mut self, display: &mut T, delta: f64, elapsed: f64) {
        let font = Vec::from(include_bytes!("font.ttf") as &[u8]);
        let mut font = FontCollection::from_bytes(font)
            .unwrap()
            .into_font()
            .unwrap();
        let mut image = RgbImage::new(display.rows() as u32, display.cols() as u32);
        let scale = Scale {
            x: 1.5 * display.rows() as f32,
            y: 1.2 * display.cols() as f32,
        };

        let mut c = ALPHA[(elapsed % 26.0) as usize];

        draw_text_mut(
            &mut image,
            Rgb([255u8, 255u8, 255u8]),
            0,
            0,
            scale,
            &font,
            &c.to_string(),
        );

        for row in 0..display.rows() {
            for col in 0..display.cols() {
                let p = image.get_pixel(row as u32, col as u32);

                display.set_at(row, col, Pixel::new(p[0], p[1], p[2], 255u8));
            }
        }
    }

    fn is_finished(&self, _display: &mut T, elapsed: f64) -> bool {
        elapsed > 26.0
    }
}
