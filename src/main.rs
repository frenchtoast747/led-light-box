extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

#[derive(Debug, Copy, Clone)]
struct Pixel(u32);

impl Default for Pixel {
    fn default() -> Self {
        Self::new(0u32, 0u32, 0u32, 0u32)
    }
}

impl From<Pixel> for u32 {
    fn from(pixel: Pixel) -> Self {
        pixel.0
    }
}

impl Pixel {
    fn new<T: Into<u32>>(r: T, g: T, b: T, w: T) -> Pixel {
        Pixel(r.into() << 24 | g.into() << 16 | b.into() << 8 | w.into())
    }

    fn r(self) -> u8 {
        ((u32::from(self) & 0xff000000) >> 24) as u8
    }

    fn g(self) -> u8 {
        ((u32::from(self) & 0x00ff0000) >> 16) as u8
    }

    fn b(self) -> u8 {
        ((u32::from(self) & 0x0000ff00) >> 8) as u8
    }

    fn w(self) -> u8 {
        (u32::from(self) & 0x0000ff) as u8
    }
}


struct Display([Pixel; 49]);

impl Default for Display {
    fn default() -> Self {
        Display([Pixel::default(); 49])
    }
}

impl Display {
    fn set_at<T: Into<usize>>(&mut self, x: T, y: T, pixel: Pixel) {
        let (x, y) = (x.into(), y.into());
        if 0 < x && x <= 7 && 0 < y && y < 7 {
            self.0[x + y * 7] = pixel;
        }
    }

    fn get_at<T: Into<usize>>(&self, x: T, y: T) -> Option<Pixel> {
        let (x, y) = (x.into(), y.into());
        if 0 <= x && x < 7 && 0 <= y && y < 7 {
            Some(self.0[x + y * 7])
        } else {
            eprintln!("Trying to access {}, {}", x, y);
            None
        }
    }
}


static BLACK: Pixel = Pixel(0x00000000u32);
static RED: Pixel = Pixel(0xff0000ffu32);

trait Animation {
    fn setup(&mut self);
    fn update(&mut self, display: &mut Display, delta: f64, elapsed: f64);
    fn is_finished(&self, elapsed: f64) -> bool;
}

struct MyAnimation {
    i: i32
}

impl MyAnimation {
    fn new(i: i32) -> Self {
        Self { i }
    }
}

impl Default for MyAnimation {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Animation for MyAnimation {
    fn setup(&mut self) {
        self.i = 0;
    }

    fn update(&mut self, display: &mut Display, delta: f64, _elapsed: f64) {
        for y in 0..7 {
            for x in 0..7 {
                let pixel = if x + y * 7 < self.i {
                    RED
                } else {
                    BLACK
                };
                display.set_at(x as usize, y as usize, pixel);
            }
        }
        self.i = (self.i + 1) % 49;
    }

    fn is_finished(&self, _elapsed: f64) -> bool {
        false
    }
}

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
    elapsed: f64,
}

impl Lightbox {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let display = &self.display;
        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(BLACK, gl);
            for y in 0..7usize {
                for x in 0..7usize {
                    let color: types::Color = display.get_at(x, y).expect("Trying to get a pixel out of bounds").into();
                    ellipse(color, [0.0, 0.0, 500.0 / 7.0, 500.0 / 7.0], ctx.transform.trans(x as f64 * 500.0 / 7.0, y as f64 * 500.0 / 7.0), gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.playlist[0].update(&mut self.display, args.dt, self.elapsed);
        self.elapsed += args.dt;
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
        playlist: vec![Box::new(MyAnimation::default())],
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

