extern crate cgmath;
extern crate glutin_window;
extern crate graphics;
extern crate num;
extern crate opengl_graphics;
extern crate piston;
extern crate rpi_ws281x_display;
extern crate simulator;
extern crate lightbox;

use rpi_ws281x_display::animations::aaron::Fireflies;
use rpi_ws281x_display::animations::snider::{BasicAnimation, CircleAnimation, StripeAnimation};
use simulator::Simulator;
use rpi_ws281x_display::animations::letters::Letters;

fn main() {
    let simulator = Simulator::new(7usize, 7usize, 720u32, 720u32);
    let mut lightbox = lightbox::LightBox::new(
        simulator,
        vec![
            Box::new(Letters::default()),
            Box::new(CircleAnimation::default()),
            Box::new(StripeAnimation::default()),
            Box::new(Fireflies::default()),
            Box::new(BasicAnimation::default()),
        ],
        30,
    );
    lightbox.run_forever();
}

