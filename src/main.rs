#[macro_use]
extern crate glium;
#[macro_use]
extern crate structopt;
extern crate nalgebra_glm as glm;
extern crate ordered_float as of;
extern crate rand;
extern crate serde;
extern crate statrs;
#[macro_use]
extern crate serde_derive;
extern crate rayon;
extern crate serde_json;

mod filter;
mod input;
mod measurement;
mod renderer;
mod state;
mod support;
mod tracker;
mod util;
mod window;

use glium::Surface;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = window::Window::from_builder(&events_loop, |win_builder, context_builder| {
        (
            win_builder.with_title("Hello There"),
            context_builder.with_vsync(true),
        )
    }).unwrap();
    let mut state = state::State::new(&window.display).expect("Failed to load state");

    let draw_parameters = glium::draw_parameters::DrawParameters {
        point_size: Some(5.0),
        blend: glium::Blend::alpha_blending(),
        ..Default::default()
    };

    let mut elapsed = 0.0;
    let mut measurement_timer = 0.0;
    support::run(
        &window,
        &mut events_loop,
        |display, target, events, time| {
            elapsed += time;
            measurement_timer += time;

            for event in events {
                if let Some(pos) = input::handle_mouse_move(event) {
                    let dims = match display.gl_window().get_inner_size() {
                        Some(dims) => dims,
                        None => break,
                    };
                    let x = pos.x / dims.width as f32 * 2.0 - 1.0;
                    let y = -pos.y / dims.height as f32 * 2.0 + 1.0;
                    state.drone_pos = glm::vec2(x, y) * 10.0;
                }
            }
            if measurement_timer >= 0.25 {
                measurement_timer -= 0.25;
                state.update(time);
            }

            state.update_renderer(display);
            target.clear_color(0.0, 0.0, 0.0, 1.0);

            state
                .renderer
                .render_to_surface(target, &draw_parameters)
                .unwrap();
        },
    );
}
