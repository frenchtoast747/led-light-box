extern crate graphics;

use framework::Pixel;
use graphics::math::Vec2d;

struct GaussianFilter {
    radius: Vec2d,
    alpha: f64,
    exp: Vec2d,
}

impl GaussianFilter {
    fn new(radius: Vec2d, alpha: f64) -> Self {
        Self {
            radius,
            alpha,
            exp: [(-alpha * radius[0] * radius[0]).exp(), (-alpha * radius[1] * radius[1]).exp()],
        }
    }

    fn evaluate(&self, p: Vec2d) -> f64 {
        return self.gaussian(p[0], self.exp[0]) * self.gaussian(p[1], self.exp[1])
    }

    fn gaussian(&self, d: f64, exp_v: f64) -> f64 {
        f64::max(0.0, (-self.alpha * d * d).exp() - exp_v)
    }
}


#[derive(Debug, Clone, Copy)]
struct BufferColor([f32; 4]);

impl Default for BufferColor {
    fn default() -> Self {
        BufferColor([0.0; 4])
    }
}

#[derive(Clone, Copy)]
struct BufferPixel {
    color: BufferColor,
    inv_scale: f32,
}

impl Default for BufferPixel {
    fn default() -> Self {
        Self { color: BufferColor::default(), inv_scale: 0.0 }
    }
}

pub struct Buffer([BufferPixel; 49]);

impl Default for Buffer {
    fn default() -> Self {
        Buffer([BufferPixel::default(); 49])
    }
}

impl Buffer {
    pub fn add_sample(&mut self, p: graphics::math::Vec2d, color: Pixel) {

    }
}
