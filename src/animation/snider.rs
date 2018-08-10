extern crate graphics;

use animation::Animation;
use buffer::{Buffer, Color, Vec2};
use framework::Display;
use framework::Pixel;
use std::ops::Rem;
use buffer::Circle;

impl From<Color> for Pixel {
    fn from(c: Color) -> Self {
        Pixel::new(
            (c.x * 255.0).round().min(255.0).max(0.0) as u8,
            (c.y * 255.0).round().min(255.0).max(0.0) as u8,
            (c.z * 255.0).round().min(255.0).max(0.0) as u8,
            (c.w * 255.0).round().min(255.0).max(0.0) as u8,
        )
    }
}

impl Buffer {
    fn apply_to_display(&self, display: &mut Display) {
        for y in 0..7usize {
            for x in 0..7usize {
                let buffer_pixel = self.at(x, y);
                display.set_at(x, y, buffer_pixel.scaled_color().into());
            }
        }
    }
}

pub struct MyAnimation {
    i: i32
}

impl MyAnimation {
    pub fn new(i: i32) -> Self {
        Self { i }
    }
}

impl Default for MyAnimation {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Animation for MyAnimation {
    fn setup(&mut self) {
        self.i = 0;
    }

    fn update(&mut self, display: &mut Display, _delta: f64, elapsed: f64) {
        let elapsed = (elapsed * 20.0).rem(49.0);
        for y in 0..7 {
            for x in 0..7 {
                let index = x + y * 7;
                let pixel = if (index as f64) < elapsed {
                    Pixel::new(255u32, 0u32, 127u32, 255u32)
                } else {
                    Pixel::default()
                };
                display.set_at(x as usize, y as usize, pixel);
            }
        }
        self.i = (self.i + 1) % 49;
    }

    fn is_finished(&self, elapsed: f64) -> bool {
        4.9 < elapsed
    }
}


#[derive(Default)]
pub struct CircleAnimation {
    buffer: Buffer,
}

impl Animation for CircleAnimation {
    fn setup(&mut self) {
        self.buffer.clear();
    }

    fn update(&mut self, display: &mut Display, delta: f64, elapsed: f64) {
        let radius = (-(elapsed as f32 / 2.0f32).cos() / 2.0 + 0.5) * 5.5;
        let circle = Circle::new(radius, Vec2::new(3.0, 3.0), Color::new(0.0, 0.5, 0.75, 1.0));
        self.buffer.clear();
        self.buffer.add_samples_grid(&circle, 8);
        self.buffer.apply_to_display(display);
    }

    fn is_finished(&self, elapsed: f64) -> bool {
        15.0 < elapsed
    }
}
