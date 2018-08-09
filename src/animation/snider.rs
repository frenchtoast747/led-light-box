extern crate graphics;

use animation::Animation;
use animation::buffer::Buffer;
use framework::Display;
use framework::Pixel;
use std::ops::Rem;

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

    fn is_finished(&self, _elapsed: f64) -> bool {
        false
    }
}

pub trait Samplable {
    fn sample(&self, p: graphics::math::Vec2d) -> Option<Pixel>;
}

struct Circle {
    radius: f64,
    origin: graphics::math::Vec2d,
    color: Pixel,
}

impl Samplable for Circle {
    fn sample(&self, p: graphics::math::Vec2d) -> Option<Pixel> {
        use graphics::math::*;
        if square_len(sub(p, self.origin)) < self.radius * self.radius {
            Some(self.color)
        } else {
            None
        }
    }
}

impl Circle {
    fn new(radius: f64, origin: graphics::math::Vec2d, color: Pixel) -> Self {
        Self { radius, origin, color }
    }

    fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    fn set_color(&mut self, color: Pixel) {
        self.color = color;
    }
}

#[derive(Default)]
pub struct CircleAnimation {
    buffer: Buffer,
}

impl Animation for CircleAnimation {
    fn setup(&mut self) {}

    fn update(&mut self, display: &mut Display, delta: f64, elapsed: f64) {
        use graphics::math::*;
        let radius = (elapsed.sin() / 2.0 + 0.5) * 3.5;
        let circle = Circle::new(radius, [3.0, 3.0], Pixel::new(0u32, 127u32, 255u32, 255u32));
        for y in 0usize..(7 * 2) {
            for x in 0usize..(7 * 2) {
                let p: Vec2d = [(x as f64) / 2.0, (y as f64) / 2.0];
                if let Some(pixel) = circle.sample(p) {
                    self.buffer.add_sample(p, pixel);
                }
            }
        }
    }

    fn is_finished(&self, elapsed: f64) -> bool {
        return elapsed < 15.0;
    }
}
