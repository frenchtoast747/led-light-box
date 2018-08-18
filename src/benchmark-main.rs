extern crate cgmath;

use animation::Animation;
use animation::snider::*;
use framework::Display;

mod framework;
mod animation;

fn main() {
    let mut d = Display::default();
    let mut a = StripeAnimation::default();
    a.setup();
    const DELTA: f64 = 0.01;
    let mut elapsed = 0.0;
    for i in 0..1000 {
        elapsed += DELTA;
        a.update(&mut d, DELTA, elapsed);
    }
}
