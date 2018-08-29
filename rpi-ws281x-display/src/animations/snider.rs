use super::super::{Animation, PixelDisplay, Pixel};
use cgmath::{InnerSpace, Matrix3, Rad, SquareMatrix, Vector2, Vector3, Vector4};
use self::helpers::*;
use std::f64::consts::PI;
use std::ops::Rem;

pub type Mat3 = Matrix3<f32>;
pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Color = Vector4<f32>;

mod helpers {
    use super::*;
    
    pub fn trunc_mod(x: f32, d: f32) -> f32 {
        x - d * (x / d).floor()
    }
    
    pub fn translation(offset: Vec2) -> Mat3 {
        Mat3::new(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            offset.x, offset.y, 1.0,
        )
    }
    
    fn _scale(factor: Vec2) -> Mat3 {
        Mat3::new(
            factor.x, 0.0, 0.0,
            0.0, factor.y, 0.0,
            0.0, 0.0, 1.0,
        )
    }
    
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
    
    pub fn super_sample_to_display<S: Sample, D: PixelDisplay>(sample: &S, elapsed: f64, display: &mut D, density: i32) {
        let samples_per_pixel = density * density;
        for y in 0usize..7 {
            for x in 0usize..7 {
                let mut color = Color::new(0.0, 0.0, 0.0, 0.0);
                for j in 1..density + 1 {
                    for i in 1..density + 1 {
                        let p = Vec2::new(
                            x as f32 + (i as f32 / (density + 1) as f32),
                            y as f32 + (j as f32 / (density + 1) as f32),
                        );
                        color += sample.sample(p, elapsed).unwrap_or(Color::new(0.0, 0.0, 0.0, 1.0));
                    }
                }
                color /= samples_per_pixel as f32;
                display.set_at(x, y, color.into());
            }
        }
    }
}

// Basic Animation

pub struct BasicAnimation {
    i: i32
}

impl Default for BasicAnimation {
    fn default() -> Self {
        BasicAnimation { i: 0 }
    }
}

impl<D: PixelDisplay> Animation<D> for BasicAnimation {
    fn setup(&mut self, _display: &mut D) {
        self.i = 0;
    }
    
    fn update(&mut self, display: &mut D, _delta: f64, elapsed: f64) {
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
    
    fn is_finished(&self, _display: &mut D, elapsed: f64) -> bool {
        4.9 < elapsed
    }
}

// Circle Animation

pub struct CircleAnimation {
    sqr_radius: f32,
    origin: Vec2,
    color1: Color,
    color2: Color,
}

impl Default for CircleAnimation {
    fn default() -> Self {
        CircleAnimation {
            sqr_radius: 1.0,
            origin: Vec2::new(3.5, 3.5),
            color1: Color::new(1.0, 0.5, 0.25, 1.0),
            color2: Color::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

impl Sample for CircleAnimation {
    fn sample(&self, p: Vec2, _t: f64) -> Option<Color> {
        let origin_to_p = p - self.origin;
        let sqr_len = origin_to_p.dot(origin_to_p);
        let factor = sqr_len / self.sqr_radius;
        if sqr_len < self.sqr_radius {
            Some(self.color1 * factor + (1.0 - factor) * self.color2)
        } else {
            None
        }
    }
}

impl<D: PixelDisplay> Animation<D> for CircleAnimation {
    fn setup(&mut self, _display: &mut D) {
        *self = CircleAnimation::default();
    }
    
    fn update(&mut self, display: &mut D, _delta: f64, elapsed: f64) {
        let radius = (-(elapsed as f32 / 1.0f32).cos() / 2.0 + 0.5) * 5.5;
        self.sqr_radius = radius * radius;
        super_sample_to_display(self, elapsed, display, 30);
    }
    
    fn is_finished(&self, _display: &mut D, elapsed: f64) -> bool {
        5.0 < elapsed
    }
}

// Stripe Animation

pub struct StripeAnimation {
    transform: Mat3,
}

impl Default for StripeAnimation {
    fn default() -> Self {
        Self {
            transform: Mat3::identity(),
        }
    }
}

pub trait Sample {
    fn sample(&self, p: Vec2, t: f64) -> Option<Color>;
}

impl Sample for StripeAnimation {
    fn sample(&self, p: Vec2, _elapsed: f64) -> Option<Color> {
        let p: Vec3 = self.transform * p.extend(1.0);
        
        if trunc_mod(p.x, 2.0) < 1.0 {
            let i = (p.x as i32 / 2).rem(4);
            match i {
                0 => Some(Color::new(1.0, 0.0, 1.0, 1.0)),
                1 => Some(Color::new(0.0, 1.0, 1.0, 1.0)),
                2 => Some(Color::new(1.0, 1.0, 0.0, 1.0)),
                _ => Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            }
        } else {
            None
        }
    }
}

impl<D: PixelDisplay> Animation<D> for StripeAnimation {
    fn setup(&mut self, _display: &mut D) {
        self.transform = Mat3::identity();
    }
    
    fn update(&mut self, display: &mut D, _delta: f64, elapsed: f64) {
        self.transform =
            {
                let elapsed = elapsed * 0.7;
                let y = elapsed * 20.0 / PI;
                let r = (y.sin() + y) / 4.0;
                translation(Vec2::new(3.5, 3.5))
                    * Mat3::from_angle_z(Rad::<f32>(r as f32))
                    * translation(Vec2::new(-3.5, -3.5))
            };
        super_sample_to_display(self, elapsed, display, 30);
    }
    
    fn is_finished(&self, _display: &mut D, elapsed: f64) -> bool {
        10.0 < elapsed
    }
}
