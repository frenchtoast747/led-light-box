extern crate cgmath;
extern crate glutin_window;
extern crate graphics;
extern crate num;
extern crate opengl_graphics;
extern crate piston;

use animation::Animation;
use animation::snider::{StripeAnimation, CircleAnimation, BasicAnimation};
use framework::{Display, Pixel};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod framework;
mod animation;


impl From<Pixel> for graphics::types::Color {
    fn from(pixel: Pixel) -> Self {
        [
            pixel.r() as f32 / 255.0,
            pixel.g() as f32 / 255.0,
            pixel.b() as f32 / 255.0,
            pixel.w() as f32 / 255.0
        ]
    }
}

pub struct Lightbox {
    gl: GlGraphics,
    display: Display,
    playlist: Vec<Box<Animation>>,
    playlist_idx: usize,
    elapsed: f64,
}

impl Lightbox {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const LED_RECT: [f64; 4] = [0.0, 0.0, 500.0 / 7.0, 500.0 / 7.0];

        let display = &self.display;
        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(BLACK, gl);
            for y in 0..7usize {
                for x in 0..7usize {
                    let color: types::Color = display.get_at(x, y).expect("Trying to get a pixel out of bounds").into();
                    ellipse(color, LED_RECT, ctx.transform.trans(x as f64 * 500.0 / 7.0, y as f64 * 500.0 / 7.0), gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let playlist_len = self.playlist.len();

        let animation = &mut self.playlist[self.playlist_idx];
        animation.update(&mut self.display, args.dt, self.elapsed);

        self.elapsed += args.dt;

        if animation.is_finished(self.elapsed) {
            use std::ops::Rem;
            self.playlist_idx = (self.playlist_idx + 1).rem(playlist_len);
            self.elapsed = 0.0;
        }

    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Lightbox Simulator",
        [500, 500],
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = Lightbox {
        gl: GlGraphics::new(opengl),
        display: Display::default(),
        playlist: vec![
            Box::new(CircleAnimation::default()),
            Box::new(StripeAnimation::default()),
            Box::new(BasicAnimation::default()),
        ],
        playlist_idx: 0,
        elapsed: 0.0,

    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

