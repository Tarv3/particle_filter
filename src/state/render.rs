use glium::backend::glutin::Display;
use glium::{DrawParameters, Program, Surface};
use renderer;
use renderer::{particle::ParticleRenderer, point::ShapeRenderer};
use std::error::Error;
use support;

pub struct StateRenderer {
    particle_color: [f32; 4],
    point_program: Program,
    particle_program: Program,

    pub point_renderer: ShapeRenderer,
    pub particle_renderer: ParticleRenderer,
}

impl StateRenderer {
    pub fn new(
        display: &Display,
        particles: usize,
        vertices: usize,
        color: [f32; 4],
    ) -> StateRenderer {
        let particle_program = support::build_particle_program(display);
        let point_program = support::build_point_program(display);
        let particle_renderer = ParticleRenderer::with_capacity(display, particles);
        let point_renderer =
            renderer::point::ShapeRenderer::with_capacity(display, vertices).unwrap();

        StateRenderer {
            particle_color: color,
            particle_program,
            point_program,

            point_renderer,
            particle_renderer,
        }
    }

    pub fn render_to_surface<S>(
        &self,
        target: &mut S,
        params: &DrawParameters,
    ) -> Result<(), Box<Error>>
    where
        S: Surface + ?Sized,
    {
        self.particle_renderer
            .render_to_surface(target, &self.particle_program, params, self.particle_color)
            .unwrap();
        self.point_renderer
            .render_to_surface(target, &self.point_program, params)?;
        Ok(())
    }
}
