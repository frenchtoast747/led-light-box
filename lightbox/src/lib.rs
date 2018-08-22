extern crate ctrlc;
extern crate rpi_ws281x_display;
extern crate time;

use rpi_ws281x_display::PixelDisplay;
use std::{thread, time as std_time};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use rpi_ws281x_display::Animation;

const FPS_IN_MS: u64 = 33;

pub struct LightBox<D: PixelDisplay> {
    pixel_display: D,
    playlist: Vec<Box<Animation<D>>>,
    playlist_idx: usize,
    elapsed: f64,
    last: f64,
    setup: bool,
}

impl<D: PixelDisplay> LightBox<D> {
    pub fn new(pixel_display: D, playlist: Vec<Box<Animation<D>>>) -> Self {
        LightBox {
            pixel_display,
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
            animation.setup(&mut self.pixel_display);
            self.setup = false;
        }

        animation.update(&mut self.pixel_display, diff, self.elapsed);

        self.elapsed += diff;
        self.pixel_display.render();

        // try to maintain the number of frames per second,
        // subtract the amount of time it took to do the last animation from
        // the total time for a specific FPS rate.
        let to_sleep: u64 = (time::precise_time_s() - now) as u64;
        // only sleep if to_sleep is less than the allotted FPS.
        // otherwise we might be sleep for a very long time...
        if to_sleep <= FPS_IN_MS {
            thread::sleep(std_time::Duration::from_millis(FPS_IN_MS - to_sleep));
        }

        if animation.is_finished(&mut self.pixel_display, self.elapsed) {
            use std::ops::Rem;
            self.playlist_idx = (self.playlist_idx + 1).rem(len);
            self.elapsed = 0.0;
            self.last = time::precise_time_s();
            self.setup = true;
        }
    }

    pub fn run_forever(&mut self) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
        self.reset();
        while running.load(Ordering::SeqCst) {
            self.update();
        }
    }
}

impl<D: PixelDisplay> Drop for LightBox<D> {
    fn drop(&mut self) {
        self.pixel_display.clear();
        self.pixel_display.render();
    }
}

