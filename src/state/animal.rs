use glm::Vec2;

pub struct Animal {
    pub color: [f32; 3],
    pub position: Vec2,
    pub signal_stddev: f32,
}

impl Animal {
    pub fn new(position: impl Into<Vec2>, signal_stddev: f32, color: [f32; 3]) -> Animal {
        Animal {
            color,
            position: position.into(),
            signal_stddev
        }
    }
}