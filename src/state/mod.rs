use glium::backend::glutin::Display;
use glm::{vec2, Vec2};
use measurement;
use rayon::prelude::*;
use renderer::SimpleVertex;
use std::error::Error;
use std::mem;
use tracker::AnimalTracker;

pub mod animal;
pub mod config;
pub mod render;

pub struct State {
    pub drone_pos: Vec2,
    pub trackers: Vec<(AnimalTracker, usize)>,
    pub renderer: render::StateRenderer,
    pub animals: Vec<animal::Animal>,
}

impl State {
    pub fn new(display: &Display) -> Result<State, Box<Error>> {
        let config = config::Config::load()?;
        let renderer =
            render::StateRenderer::new(display, config.particle_count, 12, config.particle_color);
        let mut animals = vec![];
        let mut trackers = vec![];

        for (i, animal) in config.animals_pos.iter().enumerate() {
            animals.push(animal::Animal::new(animal.0, config.stddev, animal.1));
            trackers.push((
                AnimalTracker::new(
                    config.particle_count,
                    config.init_box.h_range.clone(),
                    config.init_box.v_range.clone(),
                    config.noise,
                ),
                i,
            ));
        }

        Ok(State {
            drone_pos: vec2(0.0, 0.0),
            trackers,
            renderer,
            animals,
        })
    }

    pub fn update(&mut self, time: f32) {
        let drone_pos = self.drone_pos;
        let animals = mem::replace(&mut self.animals, vec![]);

        self.trackers.par_iter_mut().for_each(|(tracker, id)| {
            let animal = &animals[*id];
            let measurement =
                measurement::generate_measurement(drone_pos, animal.position, animal.signal_stddev);
            tracker.update_filter(drone_pos, measurement, animal.signal_stddev, time);
        });

        mem::replace(&mut self.animals, animals);

    }

    pub fn update_renderer(&mut self, display: &Display) {
        self.renderer.particle_renderer.clear_particles();
        self.renderer.point_renderer.clear_shapes();

        for (i, tracker) in self.trackers.iter_mut().enumerate() {
            let animal = &self.animals[i];
            self.renderer
                .particle_renderer
                .add_particles(tracker.0.filter.particles.iter().map(|x| SimpleVertex {
                    position: *x.as_ref(),
                }));

            self.renderer
                .point_renderer
                .add_box(tracker.0.est_pos, animal.color);
            self.renderer
                .point_renderer
                .add_box(animal.position, animal.color);
        }
        self.renderer
            .point_renderer
            .add_box(self.drone_pos, [0.0, 0.0, 1.0]);

        self.renderer
            .particle_renderer
            .update_buffer(display)
            .unwrap();

        self.renderer.point_renderer.update_buffer(display).unwrap();
    }
}
