extern crate rpi_ws281x_display;
extern crate lightbox;
extern crate rs_ws281x;

mod display;

use std::{thread, time as std_time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use rs_ws281x::StripType;
use rpi_ws281x_display::{PixelDisplay, Animation};
use rpi_ws281x_display::animations::snider::{CircleAnimation, StripeAnimation, BasicAnimation};
use rpi_ws281x_display::animations::aaron::Fireflies;

const FPS_IN_MS: u64 = 33;


fn main() {
    let display = display::GridDisplayBuilder::new()
        .gpio_pin(18)
        .brightness(50)
        .strip_type(StripType::Ws2811Rgb)
        .rows(7)
        .cols(7)
        .build();

    let mut lightbox = lightbox::LightBox::new(display, vec![
        Box::new(CircleAnimation::default()),
        Box::new(StripeAnimation::default()),
        Box::new(Fireflies::default()),
        Box::new(BasicAnimation::default()),
    ]);
    lightbox.run_forever();
}