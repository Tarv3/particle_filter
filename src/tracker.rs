use filter::Filter;
use glm::*;
use measurement::weighting;
use std::mem;
use std::ops::Range;

pub struct AnimalTracker {
    pub est_pos: Vec2,
    pub filter: Filter<Vec2>,
    pub noise: f32,
}

impl AnimalTracker {
    pub fn new(
        particles: usize,
        h_range: Range<f32>,
        v_range: Range<f32>,
        noise: f32,
    ) -> AnimalTracker {
        let filter = Filter::new_start_box(particles, h_range, v_range);
        let est_pos = filter.guess_position();

        AnimalTracker {
            est_pos,
            filter,
            noise,
        }
    }

    pub fn update_filter(&mut self, drone_pos: Vec2, measurement: f32, stddev: f32, time: f32) {
        let mut filter = mem::replace(&mut self.filter, Filter::new());
        filter.reweight(|value| weighting(drone_pos, *value, measurement, stddev));
        filter.normalize_weights();
        filter.set_sampling_weights();
        filter.resample();
        // filter.transition(|particle| *particle += self.est_vel * time);
        filter.random_movement(self.noise);

        self.est_pos = filter.guess_position();
        mem::replace(&mut self.filter, filter);
    }
}
