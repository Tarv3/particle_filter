use glm::*;
use of::OrderedFloat;
use rand::distributions::{Distribution, Uniform, Normal};
use rand;
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct Particle<T> {
    pub weight: OrderedFloat<f32>,
    pub value: T,
}

impl<T> Particle<T> {
    pub fn new(value: T) -> Self {
        Particle {
            weight: OrderedFloat(1.0),
            value
        }
    }
}

pub struct Filter<T: Copy> {
    particles: Vec<Particle<T>>,
}

impl<T: Copy> Filter<T> {
    pub fn get_particles(&self) -> &[Particle<T>] {
        self.particles.as_slice()
    }

    pub fn with_capacity(cap: usize) -> Filter<T> {
        Filter {
            particles: Vec::with_capacity(cap),
        }
    }

    pub fn populate_filter<F>(&mut self, func: F)
    where
        F: FnOnce(&mut Vec<Particle<T>>),
    {
        func(&mut self.particles);
    }

    pub fn sort_particles(&mut self) {
        self.particles.sort_by(|a, b| a.weight.cmp(&b.weight));
    }

    pub fn resample(&mut self, to_keep: usize) {
        self.sort_particles();

        let len = self.particles.len();
        if len <= to_keep {
            return;
        }

        for i in to_keep..len {
            let to_copy = i % to_keep;
            self.particles[i] = self.particles[to_copy];
        }
    }

    pub fn clear_weights(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.weight = OrderedFloat(1.0);
        }
    }

    pub fn reweight<F>(&mut self, mut func: F)
    where
        F: FnMut(&T) -> f32,
    {
        for particle in self.particles.iter_mut() {
            let weight = func(&particle.value);
            particle.weight = OrderedFloat(weight);
        }
    }

    pub fn normalize_weights(&mut self) {
        let mut sum = 0.0;
        for particle in &self.particles {
            sum += particle.weight.into_inner();
        }

        let inv_sum = 1.0 / sum;
        for particle in self.particles.iter_mut() {
            particle.weight = OrderedFloat(particle.weight.into_inner() * inv_sum);
        }
    }

    // Normalizes in the form of (1 - weight / total weight)
    pub fn reverse_normalize_weights(&mut self) {
        let mut sum = 0.0;
        for particle in &self.particles {
            sum += particle.weight.into_inner();
        }

        let inv_sum = 1.0 / sum;
        for particle in self.particles.iter_mut() {
            particle.weight = OrderedFloat(1.0 - particle.weight.into_inner() * inv_sum);
        }
    }

    pub fn transition<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut T),
    {
        for particle in self.particles.iter_mut() {
            func(&mut particle.value);
        }
    }
}

impl Filter<Vec2> {
    pub fn new_start_box(particles: usize, h_range: Range<f32>, v_range: Range<f32>) -> Filter<Vec2> {
        let h_uniform = Uniform::from(h_range);
        let v_uniform = Uniform::from(v_range);
        let mut rng = rand::thread_rng();

        let mut filter = Filter::with_capacity(particles);
        
        for _ in 0..particles {
            let x = h_uniform.sample(&mut rng);
            let y = v_uniform.sample(&mut rng);
            let pos = vec2(x, y);
            filter.particles.push(Particle::new(pos));
        }

        filter
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
            buffer[i] = particle.value.into();
        }
    }
}
