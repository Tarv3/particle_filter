use glium::{Program, program::ProgramCreationInput, Frame, glutin::{EventsLoop, Event}};
use glium::backend::glutin::Display;
use window::Window;
use std::time::Instant;

pub fn build_program(display: &Display) -> Program {
    let vertex = include_str!("shaders/vert.glsl");
    let fragment = include_str!("shaders/frag.glsl");

    let input = ProgramCreationInput::SourceCode {
        vertex_shader: vertex,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: fragment,
        transform_feedback_varyings: None,
        outputs_srgb: true,
        uses_point_size: true,
    };

    Program::new(display, input).unwrap()
}

fn get_time_and_reset(instant: &mut Instant) -> f32 {
    let now = Instant::now();
    let delta = now - *instant;
    *instant = now;

    delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0
}

pub fn run<R>(window: &Window, events_loop: &mut EventsLoop, mut func: R)
where 
    R: FnMut(&Display, &mut Frame, &mut Vec<Event>, f32)
{
    let mut events = vec![]; 
    let mut last_frame = Instant::now();

    loop {
        let delta_s = get_time_and_reset(&mut last_frame);
        let mut open = true;
        events.clear();
        events_loop.poll_events(|event| {
            if !window.closer(&event) {
                open = false;
            }
            events.push(event);
        });

        if !open {
            break;
        }

        let mut target = window.display.draw();

        func(&window.display, &mut target, &mut events, delta_s);

        target.finish().unwrap();
    }
}