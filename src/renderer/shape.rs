use renderer::Vertex;
use glm::Vec2;

pub fn build_rectangle(position: Vec2, size: [f32; 2], color: [f32; 3]) -> [Vertex; 6] {
    let left = position.x - size[0];
    let right = position.x + size[0];
    let top = position.y + size[1];
    let bottom = position.y - size[1];

    [
        Vertex { position: [right, bottom], color },
        Vertex { position: [left, top], color },
        Vertex { position: [left, bottom], color },

        Vertex { position: [right, bottom], color },
        Vertex { position: [right, top], color },
        Vertex { position: [left, top], color },
    ]
}