pub use cgmath::*;
use std::ops::AddAssign;

pub type Vec2 = Vector2<f32>;
pub type Color = Vector4<f32>;

const WIDTH: usize = 7;
const HEIGHT: usize = 7;
const NUM_PIXELS: usize = WIDTH * HEIGHT;

pub trait Samplable {
    fn sample(&self, p: Vec2) -> Option<Color>;
}

pub struct Circle {
    sqr_radius: f32,
    origin: Vec2,
    color: Color,
}

impl Samplable for Circle {
    fn sample(&self, p: Vec2) -> Option<Color> {
        let origin_to_p = p - self.origin;
        let sqr_len = dot(origin_to_p, origin_to_p);
        if sqr_len < self.sqr_radius {
            Some(self.color)
        } else {
            None
        }
    }
}

impl Circle {
    pub fn new(radius: f32, origin: Vec2, color: Color) -> Self {
        Self { sqr_radius: radius * radius, origin, color }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.sqr_radius = radius * radius;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

struct GaussianFilter {
    radius: Vec2,
    alpha: f32,
    exp: Vec2,
}

impl GaussianFilter {
    fn new(radius: Vec2, alpha: f32) -> Self {
        Self {
            radius,
            alpha,
            exp: Vector2::new((-alpha * radius.x * radius.x).exp(), (-alpha * radius.y * radius.y).exp()),
        }
    }

    fn evaluate(&self, p: Vec2) -> f32 {
        return self.gaussian(p[0], self.exp[0]) * self.gaussian(p[1], self.exp[1]);
    }

    fn gaussian(&self, d: f32, exp_v: f32) -> f32 {
        f32::max(0.0, (-self.alpha * d * d).exp() - exp_v)
    }
}

lazy_static! {
    static ref FILTER: GaussianFilter = GaussianFilter::new(Vec2::new(1.0, 1.0), 1.0);
}

#[derive(Clone, Copy)]
pub struct BufferPixel {
    color: Color,
    inv_scale: f32,
}

impl Default for BufferPixel {
    fn default() -> Self {
        BufferPixel { color: Color::new(0.0, 0.0, 0.0, 0.0), inv_scale: 0.0 }
    }
}

impl AddAssign for BufferPixel {
    fn add_assign(&mut self, rhs: BufferPixel) {
        self.color += rhs.color;
        self.inv_scale += rhs.inv_scale;
    }
}

impl BufferPixel {
    fn new(color: Color, inv_scale: f32) -> Self {
        BufferPixel { color, inv_scale }
    }

    fn clear(&mut self) {
        self.color = Color::new(0.0, 0.0, 0.0, 0.0);
        self.inv_scale = 0.0;
    }

    pub fn scaled_color(&self) -> Color {
        if self.inv_scale > 0.0 {
            self.color / self.inv_scale
        } else {
            self.color
        }
    }
}

pub struct Buffer([BufferPixel; NUM_PIXELS]);

impl Default for Buffer {
    fn default() -> Self {
        Buffer([Default::default(); NUM_PIXELS])
    }
}

impl Buffer {
    pub fn clear(&mut self) {
        for buffer_pixel in self.0.iter_mut() {
            buffer_pixel.clear();
        }
    }

    pub fn at<T: Into<usize>>(&self, x: T, y: T) -> &BufferPixel {
        &self.0[x.into() + y.into() * WIDTH]
    }

    pub fn add_sample(&mut self, p: Vec2, color: Color) {
        for y in (p.y - FILTER.radius.y).ceil() as usize
            ..(p.y + FILTER.radius.y).floor() as usize + 1 {
            for x in (p.x - FILTER.radius.x).ceil() as usize
                ..(p.x + FILTER.radius.x).floor() as usize + 1 {
                let pixel_pos = Vec2::new(x as f32, y as f32);
                let scale = FILTER.evaluate(pixel_pos - p);
                if x < WIDTH && y < HEIGHT {
                    self.0[x + y * WIDTH] += BufferPixel::new(color * scale, scale);
                }
            }
        }
    }

    pub fn add_samples_grid<S: Samplable>(&mut self, samplable: &S, density: usize) {
        for y in 0usize..(7 * density) {
            for x in 0usize..(7 * density) {
                let p = Vec2::new((x as f32) / density as f32, (y as f32) / density as f32);
                if let Some(pixel) = samplable.sample(p) {
                    self.add_sample(p.into(), pixel.into());
                } else {
                    self.add_sample(p.into(), Color::new(0.0, 0.0, 0.0, 0.0));
                }
            }
        }
    }

    pub fn add_samples_hammersley<S: Samplable>(&mut self, samplable: &S, count: u32) {
        for i in 0..count {
            let x = i as f32 / count as f32;
            let mut y = 0.0f32;
            // 1011 -> 0.1101
            // 1/2 + 1/4 + 1/8 + 0/16 + 1/32
            for j in 0..(32 - count.leading_zeros()) as u32 {
                y = y + if i & (0x1 << j) > 0 {
                    1.0f32 / 2.0f32.powi(j as i32)
                } else {
                    0.0f32
                }
            }
            let p = Vec2::new(x * 7.0, y * 7.0);
            if let Some(pixel) = samplable.sample(p) {
                self.add_sample(p.into(), pixel.into());
            } else {
                self.add_sample(p.into(), Color::new(0.0, 0.0, 0.0, 0.0));
            }

        }
    }
}
