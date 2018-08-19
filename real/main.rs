extern crate rs_ws281x;
extern crate rpi_ws281x_display;
extern crate rand;
extern crate ctrlc;
extern crate time;

mod display;

use std::{thread, time as std_time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use rs_ws281x::StripType;
use rpi_ws281x_display::{PixelDisplay, Animation};
use rpi_ws281x_display::animations::snider::{CircleAnimation, MyAnimation};
use rpi_ws281x_display::animations::aaron::{Fireflies};

use display::GridDisplay;

const FPS_IN_MS: u64 = 33;


pub struct Lightbox {
    display: GridDisplay,
    playlist: Vec<Box<Animation<GridDisplay>>>,
    playlist_idx: usize,
    elapsed: f64,
    last: f64,
    setup: bool,
}

impl Lightbox {
    pub fn new(display: GridDisplay, playlist: Vec<Box<Animation<GridDisplay>>>) -> Self {
        Lightbox {
            display,
            playlist,
            playlist_idx: 0,
            elapsed: 0.0,
            last: 0.0,
            setup: true,
        }
    }

    pub fn reset(&mut self) {
        self.playlist_idx = 0;
        self.elapsed = 0.0;
        self.last = time::precise_time_s();
        self.setup = true;
    }

    pub fn update(&mut self) {
        let len = self.playlist.len();
        let now = time::precise_time_s();
        let diff = now - self.last;
        self.last = now;

        let animation = &mut self.playlist[self.playlist_idx];
        if self.setup {
            animation.setup(&mut self.display);
            self.setup = false;
        }

        animation.update(&mut self.display, diff, self.elapsed);

        self.elapsed += diff;
        self.display.render();

        // try to maintain the number of frames per second,
        // subtract the amount of time it took to do the last animation from
        // the total time for a specific FPS rate.
        let to_sleep: u64 = (time::precise_time_s() - now) as u64;
        // only sleep if to_sleep is less than the allotted FPS.
        // otherwise we might be sleep for a very long time...
        if to_sleep <= FPS_IN_MS {
            thread::sleep(std_time::Duration::from_millis(FPS_IN_MS - to_sleep));
        }

        if animation.is_finished(&mut self.display, self.elapsed) {
            use std::ops::Rem;
            self.playlist_idx = (self.playlist_idx + 1).rem(len);
            self.elapsed = 0.0;
            self.last = time::precise_time_s();
            self.setup = true;
        }
    }
}

impl Drop for Lightbox {
    fn drop(&mut self) {
        self.display.clear();
        self.display.render();
    }
}

fn main() {
    println!("Starting!");

    let display = display::GridDisplayBuilder::new()
        .gpio_pin(18)
        .brightness(50)
        .strip_type(StripType::Ws2811Rgb)
        .rows(7)
        .cols(7)
        .build();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut lightbox = Lightbox::new(
        display,
        vec![
//            Box::new(CircleAnimation::default()),
//            Box::new(MyAnimation::default()),
            Box::new(Fireflies::default()),
        ]);

    lightbox.reset();
    while running.load(Ordering::SeqCst) {
        lightbox.update();
    }

    println!("Stopping!");
}
