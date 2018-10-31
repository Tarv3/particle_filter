use glm::*;
use rand::distributions::{Normal, Distribution};

// Returns z score of the distance between the particle and the drone
pub fn weighting(drone: Vec2, particle: Vec2, measurement: f32, stddev: f32) -> f32 {
    let length = distance(&drone, &particle);
    let score = (length - measurement) / stddev; 

    score.abs()
}

pub fn generate_measurement(drone: Vec2, animal: Vec2, stddev: f32) -> f32 {
    let distance = distance(&drone, &animal);
    let normal = Normal::new(0.0, stddev as f64);

    distance + normal.sample(&mut rand::thread_rng()) as f32
}