use std::f64::consts::PI;

use rand::{Rng, thread_rng};

use super::super::{Animation, PixelDisplay, Pixel};

const TWO_PI: f64 = PI * 2.0;
const HALF_PI: f64 = PI / 2.0;

#[derive(Debug)]
struct Ball {
    id: u32,
    pos_x: usize,
    pos_y: usize,
    color: Pixel,
    elapsed: f64,
    ttl: f64,
}

impl Ball {
    pub fn new(id: u32, rows: usize, cols: usize) -> Self {
        let mut rng = thread_rng();
        Ball {
            id,
            pos_x: rng.gen_range(0, cols),
            pos_y: rng.gen_range(0, rows),
            color: Pixel(rng.gen()),
            elapsed: 0.0,
            ttl: rng.gen_range(1.0, 60.0),
        }
    }

    pub fn update<T: PixelDisplay>(&mut self, display: &mut T, delta: f64) {
        self.elapsed += delta;
        let ratio = (self.elapsed / self.ttl).min(1.0);

        let brightness = cycle(255.0, ratio, 255.0) as u8;
        let pos_x = cycle((display.cols() - 1) as f64, ratio, self.pos_x as f64) as usize;
        let pos_y = cycle((display.rows() - 1) as f64, ratio, self.pos_y as f64) as usize;

        display.set_at(pos_x, pos_y, self.color.at_brightness(brightness));
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed > self.ttl
    }
}

fn cycle(a: f64, ratio: f64, start: f64) -> f64 {
    let p = ((start / a) * PI) - HALF_PI;
    let x = ratio * TWO_PI;
    ((a * (x - p).sin()) + a) * 0.5
}


#[derive(Default)]
pub struct Fireflies {
    ball_id: u32,
    balls: Vec<Ball>,
}

impl<T: PixelDisplay> Animation<T> for Fireflies {
    fn setup(&mut self, _display: &mut T) {
        self.balls = Vec::new();
        self.ball_id = 0;
    }

    fn update(&mut self, display: &mut T, delta: f64, _elapsed: f64) {
        if self.balls.len() < 10 {
            let b = Ball::new(self.ball_id, display.rows(), display.cols());
            self.ball_id += 1;
            self.balls.push(b);
        }

        // fade out all of the pixels to imitate trails
        for row in 0..display.rows() {
            for col in 0..display.cols() {
                let p = display.get_at(col, row);
                display.set_at(col, row, p.scale(0.90));
            }
        }

        for ball in self.balls.iter_mut() {
            ball.update(display, delta);
        }

        self.balls.retain(|b| !b.is_finished());
    }

    fn is_finished(&self, _display: &mut T, elapsed: f64) -> bool {
        elapsed > 30.0
    }
}
