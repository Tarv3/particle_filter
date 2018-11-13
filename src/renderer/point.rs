use glium::backend::glutin::Display;
use glium::{
    draw_parameters::DrawParameters,
    index::{NoIndices, PrimitiveType},
    vertex::BufferCreationError,
    Program, Surface, VertexBuffer,
};
use renderer::Vertex;
use std::error::Error;
use glm::Vec2;
use renderer::shape;

pub struct ShapeRenderer {
    vertices: Vec<Vertex>,
    buffer: VertexBuffer<Vertex>,
}

impl ShapeRenderer {
    pub fn clear_shapes(&mut self) {
        self.vertices.clear();
    }

    pub fn with_capacity(display: &Display, cap: usize) -> Result<ShapeRenderer, BufferCreationError> {
        Ok(ShapeRenderer {
            vertices: Vec::with_capacity(cap),
            buffer: VertexBuffer::empty_dynamic(display, cap)?
        })
    } 

    pub fn add_vertices(&mut self, vertices: impl Iterator<Item = Vertex>) {
        self.vertices.extend(vertices);
    }

    pub fn add_box(&mut self, position: Vec2, color: [f32; 3]) {
        let shape = shape::build_rectangle(position, [0.25; 2], color);
        self.add_vertices(shape.iter().map(|x| *x));
    }

    pub fn update_buffer(&mut self, display: &Display) -> Result<(), BufferCreationError> {
        self.buffer.invalidate();

        if self.buffer.len() < self.vertices.len() {
            self.buffer = VertexBuffer::dynamic(display, &self.vertices)?;
        } else {
            let slice = self.buffer.slice(0..self.vertices.len()).unwrap();
            slice.write(&self.vertices);
        }

        Ok(())
    }

    pub fn render_to_surface<S>(
        &self,
        target: &mut S,
        program: &Program,
        params: &DrawParameters,
    ) -> Result<(), Box<Error>>
    where
        S: Surface + ?Sized,
    {
        let uniforms = uniform!();
        let slice = self.buffer.slice(0..self.vertices.len()).unwrap();

        target.draw(
            slice,
            NoIndices(PrimitiveType::TrianglesList),
            program,
            &uniforms,
            params,
        )?;
        Ok(())
    }
}
