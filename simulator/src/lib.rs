extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rpi_ws281x_display;

use piston::input::{Input, Event};
use glutin_window::{GlutinWindow, OpenGL};
use graphics::types::Color as PistonColor;
use graphics::Viewport;
use opengl_graphics::GlGraphics;
use piston::window::{Window, WindowSettings};
use rpi_ws281x_display::{Pixel, PixelDisplay};

struct MyPixel(Pixel);

impl From<Pixel> for MyPixel {
    fn from(pixel: Pixel) -> Self {
        MyPixel(pixel)
    }
}

impl From<MyPixel> for PistonColor {
    fn from(pixel: MyPixel) -> Self {
        [
            pixel.0.r() as f32 / 255.0,
            pixel.0.g() as f32 / 255.0,
            pixel.0.b() as f32 / 255.0,
            pixel.0.w() as f32 / 255.0
        ]
    }
}

impl From<PistonColor> for MyPixel {
    fn from(color: PistonColor) -> Self {
        MyPixel(Pixel::new((color[0] * 255.0f32) as u32,
                           (color[1] * 255.0f32) as u32,
                           (color[2] * 255.0f32) as u32,
                           (color[3] * 255.0f32) as u32,
        ))
    }
}

pub struct Simulator {
    graphics: GlGraphics,
    window: GlutinWindow,
    rows: usize,
    cols: usize,
    buffer: Vec<PistonColor>,
    viewport_full: Viewport,
    pub width: u32,
    pub height: u32,
}

impl PixelDisplay for Simulator {
    fn rows(&self) -> usize {
        self.rows
    }
    fn cols(&self) -> usize {
        self.cols
    }

    fn set_at<T: Into<usize>>(&mut self, x: T, y: T, pixel: Pixel) {
        let (x, y) = (x.into(), y.into());
        self.buffer[x + y * self.cols] = MyPixel(pixel).into();
    }

    fn get_at<T: Into<usize>>(&self, x: T, y: T) -> Pixel {
        let (x, y) = (x.into(), y.into());
        MyPixel::from(self.buffer[x + y * self.cols]).0
    }

    fn get_brightness(&self) -> u8 {
        return 255;
    }

    fn set_brightness(&mut self, brightness: u8) {

    }

    fn render(&mut self) {
        self.flush_input();

        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let LED_RECT: [f64; 4] = [0.0, 0.0, self.width as f64 / self.cols() as f64, self.height as f64 / self.rows() as f64];

        let ctx = self.graphics.draw_begin(self.viewport_full);
        clear(BLACK, &mut self.graphics);
        for y in 0..self.rows() {
            for x in 0..self.cols() {
                let mp: MyPixel = self.get_at(x, y).into();
                let color: types::Color = mp.into();
                ellipse(color, LED_RECT, ctx.transform.trans(x as f64 * self.width as f64 / self.cols() as f64, y as f64 * self.height as f64 / self.rows() as f64), &mut self.graphics);
            }
        }
        self.graphics.draw_end();
        self.window.swap_buffers();
    }

    fn clear(&mut self) {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                self.set_at(col, row, Pixel::new(0u8, 0u8, 0u8, 0u8));
            }
        }
    }
}

impl Simulator {
    pub fn new<T: Into<usize>, U: Into<u32>>(cols: T, rows: T, width: U, height: U) -> Self {
        let (cols, rows, width, height) = (cols.into(), rows.into(), width.into(), height.into());
        let opengl = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new(
            "Lightbox Simulator",
            [width, height],
        )
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .expect("Error creating window for Lightbox Simulator");
        let mut buffer = vec![];
        buffer.reserve(cols * rows);
        for _ in 0..(cols * rows) {
            buffer.push([0.0, 0.0, 0.0, 1.0]);
        }
        let window_size = window.size();
        let draw_size = window.draw_size();
        let viewport_full = Viewport {
            rect: [0, 0, draw_size.width as i32, draw_size.height as i32],
            draw_size: draw_size.into(),
            window_size: window_size.into(),
        };
        Self {
            graphics: GlGraphics::new(opengl),
            window,
            viewport_full,
            rows: rows.into(),
            cols: cols.into(),
            buffer,
            width,
            height,
        }
    }

    fn flush_input(&mut self) {
        loop {
            let thing = Window::poll_event(&mut self.window);
            match Window::poll_event(&mut self.window) {
                Some(Event::Input(Input::Resize(_), _)) => {
                    let window_size = self.window.size();
                    let draw_size = self.window.draw_size();
                    self.viewport_full = Viewport {
                        rect: [0, 0, draw_size.width as i32, draw_size.height as i32],
                        draw_size: draw_size.into(),
                        window_size: window_size.into(),
                    }
                }
                None => break,
                _ => {}
            }
        }
    }
}
