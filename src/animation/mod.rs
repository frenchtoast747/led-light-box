use framework::Display;

pub mod snider;

pub trait Animation {
    fn setup(&mut self);
    fn update(&mut self, display: &mut Display, delta: f64, elapsed: f64);
    fn is_finished(&self, elapsed: f64) -> bool;
}

