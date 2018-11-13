use glm::*;
use of::OrderedFloat;
use rand;
use rand::distributions::{Distribution, Normal, Uniform};
use std::mem::swap;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Filter<T: Copy> {
    pub particles: Vec<T>,
    old_particles: Vec<T>,

    pub weights: Vec<OrderedFloat<f32>>,
}

impl<T: Copy> Filter<T> {
    pub fn new() -> Filter<T> {
        Filter {
            particles: vec![],
            old_particles: vec![],

            weights: vec![],
        }
    }

    pub fn get_particles(&self) -> &[T] {
        self.particles.as_slice()
    }

    pub fn with_capacity(cap: usize) -> Filter<T> {
        Filter {
            particles: Vec::with_capacity(cap),
            old_particles: Vec::with_capacity(cap),
            weights: Vec::with_capacity(cap),
        }
    }

    pub fn populate_filter<F>(&mut self, func: F)
    where
        F: FnOnce(&mut Vec<T>),
    {
        func(&mut self.particles);
    }

    pub fn resample(&mut self) {
        assert!(self.weights.len() == self.particles.len());
        swap(&mut self.particles, &mut self.old_particles);
        self.particles.clear();

        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(0.0, 1.0);
        let len = self.old_particles.len();

        for _ in 0..len {
            let mut index = len;
            while index >= len {
                let num = uniform.sample(&mut rng);
                index = match self.weights.binary_search(&OrderedFloat(num)) {
                    Ok(index) => index,
                    Err(index) => index,
                };
            }

            self.particles.push(self.old_particles[index]);
        }
    }

    pub fn reset<F>(&mut self, mut resampler: F)
    where
        F: FnMut() -> T,
    {
        let len = self.particles.len();
        self.particles.clear();
        for _ in 0..len {
            self.particles.push(resampler());
        }
    }

    pub fn clear_weights(&mut self) {
        self.weights.clear();
    }

    pub fn reweight<F>(&mut self, mut func: F)
    where
        F: FnMut(&T) -> f32,
    {
        self.clear_weights();
        for particle in self.particles.iter_mut() {
            self.weights.push(func(&particle).into());
        }
    }

    pub fn normalize_weights(&mut self) {
        let sum: f32 = self.weights.iter().map(|x| x.into_inner()).sum();
        let one_over_sum = 1.0 / sum;
        self.weights
            .iter_mut()
            .for_each(|x| *x = OrderedFloat(x.into_inner() * one_over_sum));
    }

    pub fn sum_weights(&self) -> f32 {
        self.weights.iter().map(|x| x.into_inner()).sum()
    }

    pub fn set_sampling_weights(&mut self) {
        let mut sum = 0.0;
        self.weights.iter_mut().for_each(|x| {
            sum += x.into_inner();
            *x = OrderedFloat(sum);
        });
    }

    pub fn transition<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut T),
    {
        for particle in self.particles.iter_mut() {
            func(particle);
        }
    }
}

impl Filter<Vec2> {
    pub fn new_start_box(
        particles: usize,
        h_range: Range<f32>,
        v_range: Range<f32>,
    ) -> Filter<Vec2> {
        let h_uniform = Uniform::from(h_range);
        let v_uniform = Uniform::from(v_range);
        let mut rng = rand::thread_rng();

        let mut filter = Filter::with_capacity(particles);

        for _ in 0..particles {
            let x = h_uniform.sample(&mut rng);
            let y = v_uniform.sample(&mut rng);
            let pos = vec2(x, y);
            filter.particles.push(pos);
        }

        filter
    }

    pub fn reset_uniformly(&mut self, h_range: Range<f32>, v_range: Range<f32>) {
        let h_uniform = Uniform::from(h_range);
        let v_uniform = Uniform::from(v_range);
        let mut rng = rand::thread_rng();

        self.reset(|| {
            let x = h_uniform.sample(&mut rng);
            let y = v_uniform.sample(&mut rng);
            vec2(x, y)
        });
    }

    pub fn guess_position(&self) -> Vec2 {
        let len = self.particles.len();
        let one_on_len = 1.0 / len as f32;

        self.particles
            .iter()
            .fold(vec2(0.0, 0.0), |avg, x| avg + *x * one_on_len)
    }

    pub fn random_movement(&mut self, stddev: f32) {
        let normal = Normal::new(0.0, stddev.into());
        let mut rng = rand::thread_rng();
        self.transition(|pos| {
            let x = normal.sample(&mut rng) as f32;
            let y = normal.sample(&mut rng) as f32;
            *pos += vec2(x, y);
        })
    }

    pub fn write_particle_pos(&self, buffer: &mut [[f32; 2]]) {
        assert!(buffer.len() >= self.particles.len());

        for (i, particle) in self.particles.iter().enumerate() {
            buffer[i] = *particle.as_ref();
        }
    }
}
