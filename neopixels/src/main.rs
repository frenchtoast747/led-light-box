#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate rpi_ws281x_display;
extern crate lightbox;
extern crate rs_ws281x;

mod display;
use display::{GridDisplay, GridDisplayBuilder};

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Thread;
use std::sync::atomic::AtomicPtr;
use std::ops::Deref;

use rocket_contrib::Json;

use rocket::State;
use rs_ws281x::StripType;
use rpi_ws281x_display::PixelDisplay;

//use rpi_ws281x_display::animations::snider::{CircleAnimation, StripeAnimation, BasicAnimation};
use rpi_ws281x_display::animations::aaron::Fireflies;
use lightbox::LightBox;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::ops::DerefMut;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

type ManagerState<'a> = State<'a, Arc<Mutex<Manager>>>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/power/status")]
fn power_status(manager: ManagerState) -> Json {
    return Json(json!({
        "status": manager.lock().unwrap().running.load(Ordering::SeqCst),
    }));
}

#[post("/power/update/<on_off>")]
fn set_power(on_off: bool, manager: ManagerState) -> Json {
    let manager_t = manager.clone();
    let manager = manager.lock().unwrap();
    if on_off {
        manager.running.store(true, Ordering::SeqCst);
        let running_t = manager.running.clone();
        thread::spawn(move || {
            {
                manager_t.lock().unwrap().lightbox.reset();
            }
            while running_t.load(Ordering::SeqCst) {
                manager_t.lock().unwrap().lightbox.update();
            }
            manager_t.lock().unwrap().lightbox.clear();
        });

    } else {
        manager.running.store(false, Ordering::SeqCst);
    }
    return Json(json!({
        "status": on_off,
    }));
}

#[get("/brightness/status")]
fn brightness_status(manager: ManagerState) -> Json {
    let brightness = manager.lock().unwrap().lightbox.display.get_brightness();
    let percentage = brightness_to_percentage(brightness);
    return Json(json!({
        "status": percentage,
    }));
}

#[post("/brightness/update/<percentage>")]
fn set_brightness(percentage: u8, manager: ManagerState) -> Json {
    let brightness = percentage_to_brightness(percentage);
    manager.lock().unwrap().lightbox.display.set_brightness(brightness);
    return Json(json!({
        "status": percentage,
    }));
}

fn brightness_to_percentage(brightness: u8) -> u8 {
    (brightness as f64 / 255.0 * 100.0) as u8
}

fn percentage_to_brightness(percentage: u8) -> u8 {
    (percentage as f64 * 255.0 / 100.0) as u8
}

struct MyLightBox(LightBox<GridDisplay>);

impl Deref for MyLightBox {
    type Target = LightBox<GridDisplay>;
    fn deref(&self) -> &LightBox<GridDisplay> {
        return &self.0;
    }
}

impl DerefMut for MyLightBox {
    fn deref_mut(&mut self) -> &mut LightBox<GridDisplay> {
        return &mut self.0;
    }
}
unsafe impl Send for MyLightBox {}

struct Manager {
    pub lightbox: MyLightBox,
    pub thread: Option<Thread>,
    pub running: Arc<AtomicBool>,
}

//unsafe impl Send for Manager {}

fn main() {
    let display = GridDisplayBuilder::new()
        .gpio_pin(18)
        .brightness(50)
        .strip_type(StripType::Ws2811Rgb)
        .rows(7)
        .cols(7)
        .build();

    let lightbox = LightBox::new(
        display, vec![
            // Box::new(CircleAnimation::default()),
            // Box::new(StripeAnimation::default()),
            Box::new(Fireflies::default()),
            // Box::new(BasicAnimation::default()),
        ],
        30,
    );

    let manager = Manager {lightbox: MyLightBox(lightbox), thread: None, running: Arc::new(AtomicBool::new(false))};

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    {
        rocket::ignite()
            .attach(cors.to_cors().unwrap())
            .mount("/", routes![index, power_status, set_power, brightness_status, set_brightness])
            .manage(Arc::new(Mutex::new(manager)))
            .launch();
    }

}
