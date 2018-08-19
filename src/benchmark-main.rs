extern crate cgmath;
extern crate rpi_ws281x_display;

mod display;

use rpi_ws281x_display::Animation;
use rpi_ws281x_display::animations::snider::*;
use display::Display;

fn main() {
    let mut d = Display::default();
    let mut a = StripeAnimation::default();
    a.setup(&mut d);
    const DELTA: f64 = 0.01;
    let mut elapsed = 0.0;
    for i in 0..1000 {
        elapsed += DELTA;
        a.update(&mut d, DELTA, elapsed);
    }
}
