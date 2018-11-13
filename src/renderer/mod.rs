
pub mod particle;
pub mod point;
pub mod shape;

#[derive(Copy, Clone, Debug)]
pub struct SimpleVertex {
    pub position: [f32; 2],
}

implement_vertex!(SimpleVertex, position);

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);
