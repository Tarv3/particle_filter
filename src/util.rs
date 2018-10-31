use filter::Filter;
use measurement::*;
use glm::*;

pub fn update_filter(filter: &mut Filter<Vec2>, drone: Vec2, animal: Vec2, stddev: f32) {
    let measurement = generate_measurement(drone, animal, stddev);
    filter.reweight(|value| {
        weighting(drone, *value, measurement, stddev)
    });
    filter.normalize_weights();
    filter.resample(50);
    filter.random_movement(0.5);
}


