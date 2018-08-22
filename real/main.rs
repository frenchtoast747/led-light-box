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


