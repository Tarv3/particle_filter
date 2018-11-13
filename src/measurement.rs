use glm::*;
use rand::distributions::{Distribution, Normal};
use statrs::function::erf;
use std::f32::consts::FRAC_1_SQRT_2;

// Returns z score of the distance between the particle and the drone
pub fn weighting(drone: Vec2, particle: Vec2, measurement: f32, stddev: f32) -> f32 {
    let length = distance(&drone, &particle);
    let score = (length - measurement) / stddev;
    let value = 1.0 - erf::erf((score.abs() * FRAC_1_SQRT_2) as f64) as f32;
    value
}

pub fn generate_measurement(drone: Vec2, animal: Vec2, stddev: f32) -> f32 {
    let distance = distance(&drone, &animal);
    let normal = Normal::new(0.0, stddev as f64);

    distance + normal.sample(&mut rand::thread_rng()) as f32
}
