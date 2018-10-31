#[macro_use]
extern crate glium;
extern crate rand;
extern crate nalgebra_glm as glm;
extern crate ordered_float as of;

mod window;
mod support;
mod measurement;
mod renderer;
mod filter;
mod util;

use glium::Surface;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let mut window = window::Window::from_builder(&events_loop, |win_builder, context_builder| {
        (win_builder.with_title("Hello There"), context_builder.with_vsync(true))
    }).unwrap();

    let program = support::build_program(&window.display);
    let mut renderer = renderer::ParticleRenderer::with_capacity(&window.display, 1000);
    let mut filter = filter::Filter::new_start_box(1000, -10.0..10.0, -10.0..10.0);

    let animal_pos = glm::vec2(0.0, 0.0);
    let mut drone_pos = glm::vec2(5.0, 1.0);
    let stddev = 0.01;
    let motion = glm::vec2(-1.0, 1.0);

    let draw_parameters = glium::draw_parameters::DrawParameters {
        point_size: Some(3.0),
        .. Default::default()
    };

    support::run(&window, &mut events_loop, |display, target, _events, time| {
        drone_pos += motion * time;
        util::update_filter(&mut filter, drone_pos, animal_pos, stddev);
        renderer.update_particles(&filter);
        renderer.update_buffer(&window.display).unwrap();

        target.clear_color(0.0, 0.0, 0.0, 1.0);
        renderer.render_to_surface(target, &program, &draw_parameters, [1.0, 0.0, 0.0, 1.0]);

    });
}
