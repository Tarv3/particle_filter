use glium::{VertexBuffer, vertex::BufferCreationError, Surface, Program, index::{NoIndices, PrimitiveType}, draw_parameters::DrawParameters};
use glium::backend::glutin::Display;
use filter::Filter;
use glm::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct ParticleRenderer {
    particles: Vec<Vertex>,
    buffer: VertexBuffer<Vertex>,
}

impl ParticleRenderer {
    pub fn with_capacity(display: &Display, cap: usize) -> ParticleRenderer {
        let particles = Vec::with_capacity(cap);
        let buffer = VertexBuffer::empty_dynamic(display, cap).unwrap();

        ParticleRenderer {
            particles,
            buffer,
        }
    }

    pub fn update_particles(&mut self, filter: &Filter<Vec2>) {
        self.particles.clear();
        for particle in filter.get_particles().iter() {
            self.particles.push(Vertex { position: particle.value.into() });
        }
    }

    pub fn update_buffer(&mut self, display: &Display) -> Result<(), BufferCreationError>{
        self.buffer.invalidate();
        if self.buffer.len() < self.particles.len() {
            self.buffer = VertexBuffer::dynamic(display, &self.particles)?;
        }
        else {
            let slice = self.buffer.slice(0..self.particles.len()).unwrap();
            slice.write(&self.particles);
        }

        Ok(())
    }

    pub fn render_to_surface<S>(&self, target: &mut S, program: &Program, params: &DrawParameters, color: [f32; 4])
    where 
        S: Surface + ?Sized 
    {
        let uniforms = uniform!(u_color: color);
        let slice = self.buffer.slice(0..self.particles.len()).unwrap();

        target.draw(slice, NoIndices(PrimitiveType::Points), program, &uniforms, params);
    }
}