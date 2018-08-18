use framework;

pub mod snider;

pub trait Animation {
    fn setup(&mut self);
    fn update(&mut self, display: &mut framework::Display, delta: f64, elapsed: f64);
    fn is_finished(&self, elapsed: f64) -> bool;
}

