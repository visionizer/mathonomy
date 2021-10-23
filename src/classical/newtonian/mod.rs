use crate::prelude::*;

pub fn sforce(mass: f64, accel: f64) -> f64 {
    mass * accel
}

pub fn smass(force: f64, accel: f64) -> f64 {
    force / accel
}

pub fn saccel(force: f64, mass: f64) -> f64 {
    force / mass
}
