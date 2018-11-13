use glium::glutin::{Event, WindowEvent};
use glm::*;

pub fn handle_mouse_move(event: &Event) -> Option<Vec2> {
    match event {
        Event::WindowEvent { event, .. } => match event {
           WindowEvent::CursorMoved { position, .. } => {
               Some(vec2(position.x as f32, position.y as f32))
           } 
           _ => None,
        }
        _ => None
    }
}