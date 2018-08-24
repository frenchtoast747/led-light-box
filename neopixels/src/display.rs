use rs_ws281x::{Controller, ControllerBuilder, ChannelBuilder};
use rs_ws281x::StripType;

use rpi_ws281x_display::{PixelDisplay, Pixel};

pub struct GridDisplayBuilder {
    cb: ControllerBuilder,
    chb: ChannelBuilder,
    rows: usize,
    cols: usize,
}

impl GridDisplayBuilder {
    pub fn new() -> Self {
        GridDisplayBuilder {
            cb: ControllerBuilder::new(),
            chb: ChannelBuilder::new(),
            rows: 0,
            cols: 0,
        }
    }

    pub fn rows(&mut self, r: usize) -> &mut Self {
        self.rows = r;
        self
    }

    pub fn cols(&mut self, c: usize) -> &mut Self {
        self.cols = c;
        self
    }

    pub fn freq(&mut self, freq: u32) -> &mut Self {
        self.cb.freq(freq);
        self
    }

    pub fn dma(&mut self, dmanum: i32) -> &mut Self {
        self.cb.dma(dmanum);
        self
    }

    pub fn gpio_pin(&mut self, pin: i32) -> &mut Self {
        self.chb.pin(pin);
        self
    }

    pub fn brightness(&mut self, b: u8) -> &mut Self {
        self.chb.brightness(b);
        self
    }

    pub fn invert(&mut self, val: bool) -> &mut Self {
        self.chb.invert(val);
        self
    }

    pub fn strip_type(&mut self, strip: StripType) -> &mut Self {
        self.chb.strip_type(strip);
        self
    }

    pub fn build(&mut self) -> GridDisplay {
        let led_count = (self.rows * self.cols) as i32;
        self.chb.count(led_count);
        self.cb.channel(0, self.chb.build());

        GridDisplay {
            controller: self.cb.build().expect("Failed to create controller"),
            rows: self.rows,
            cols: self.cols,
        }
    }
}


pub struct GridDisplay {
    controller: Controller,
    pub rows: usize,
    pub cols: usize,
}

impl GridDisplay {
    fn x_y_to_idx<T: Into<usize>>(&self, x: T, y: T) -> usize {
        let row = y.into();
        let col = x.into();
        let is_reversed = row % 2 != 0;
        if is_reversed {
            return (((row + 1) * self.rows) - col - 1).into();
        }
        ((row * self.rows) + col).into()
    }
}

impl PixelDisplay for GridDisplay {
    fn rows(&self) -> usize {
        return self.rows;
    }

    fn cols(&self) -> usize {
        return self.cols;
    }

    fn set_at<T: Into<usize>>(&mut self, x: T, y: T, pixel: Pixel) {
        let (x, y) = (x.into(), y.into());
        if x < self.cols && y < self.rows {
            let idx = self.x_y_to_idx(x, y);
            let leds = self.controller.leds_mut(0);
            leds[idx] = [pixel.b(), pixel.g(), pixel.r(), pixel.w()];
        }
    }

    fn get_at<T: Into<usize>>(&self, x: T, y: T) -> Pixel {
        let idx = self.x_y_to_idx(x.into(), y.into());
        let leds = self.controller.leds(0);
        let rc = leds[idx];
        Pixel::new(rc[2], rc[1], rc[0], rc[3])
    }

    fn render(&mut self) {
        self.controller.render();
    }

    fn clear(&mut self) {
        for col in 0..self.cols {
            for row in 0..self.rows {
                self.set_at(col, row, Pixel::default())
            }
        }
    }
}
