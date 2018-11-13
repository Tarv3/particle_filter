use filter::Filter;
use glium::backend::glutin::Display;
use glium::{
    draw_parameters::DrawParameters,
    index::{NoIndices, PrimitiveType},
    vertex::BufferCreationError,
    Program, Surface, VertexBuffer,
};
use glm::Vec2;
use std::error::Error;
use renderer::SimpleVertex;


pub struct ParticleRenderer {
    particles: Vec<SimpleVertex>,
    buffer: VertexBuffer<SimpleVertex>,
}

impl ParticleRenderer {
    pub fn with_capacity(display: &Display, cap: usize) -> ParticleRenderer {
        let particles = Vec::with_capacity(cap);
        let buffer = VertexBuffer::empty_dynamic(display, cap).unwrap();

        ParticleRenderer { particles, buffer }
    }

    pub fn update_particles(&mut self, filter: &Filter<Vec2>) {
        self.particles.clear();
        for particle in filter.get_particles().iter() {
            self.particles.push(SimpleVertex {
                position: *particle.as_ref(),
            });
        }
    }

    pub fn clear_particles(&mut self) {
        self.particles.clear();
    }

    pub fn add_particles(&mut self, particles: impl Iterator<Item = SimpleVertex>) {
        self.particles.extend(particles);
    }

    pub fn update_buffer(&mut self, display: &Display) -> Result<(), BufferCreationError> {
        self.buffer.invalidate();
        if self.buffer.len() < self.particles.len() {
            self.buffer = VertexBuffer::dynamic(display, &self.particles)?;
        } else {
            let slice = self.buffer.slice(0..self.particles.len()).unwrap();
            slice.write(&self.particles);
        }

        Ok(())
    }

    pub fn render_to_surface<S>(
        &self,
        target: &mut S,
        program: &Program,
        params: &DrawParameters,
        color: [f32; 4],
    ) -> Result<(), Box<Error>>
    where
        S: Surface + ?Sized,
    {
        let uniforms = uniform!(u_color: color);
        let slice = self.buffer.slice(0..self.particles.len()).unwrap();

        target.draw(
            slice,
            NoIndices(PrimitiveType::Points),
            program,
            &uniforms,
            params,
        )?;
        Ok(())
    }
}
