#![feature(test)]
extern crate test;

extern crate cgmath;
extern crate rpi_ws281x_display;
extern crate simulator;

use rpi_ws281x_display::Animation;
use rpi_ws281x_display::animations::snider::*;
use simulator::Simulator;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_stripe_animation(b: &mut Bencher) {
        let mut d = Simulator::new(7usize, 7usize);
        let mut a = StripeAnimation::default();
        a.setup(&mut d);
        const DELTA: f64 = 0.01;
        let mut elapsed = 0.0;
        b.iter(|| {
            elapsed += DELTA;
            a.update(&mut d, DELTA, elapsed);
        });
    }
}
