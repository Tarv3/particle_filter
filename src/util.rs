use filter::Filter;
use glium::backend::glutin::Display;
use glm::*;
use measurement::*;
use renderer::point::ShapeRenderer;
use renderer::shape::*;
use std::fs::File;
use state::config::Config;
use serde_json;

pub fn update_filter(filter: &mut Filter<Vec2>, drone: Vec2, animal: Vec2, stddev: f32) {
    let measurement = generate_measurement(drone, animal, stddev);
    filter.reweight(|value| weighting(drone, *value, measurement, stddev));
    filter.normalize_weights();
    filter.set_sampling_weights();
    filter.resample();
    filter.random_movement(0.5);
}

pub fn elipse_motion(elapsed: f32, time: f32, vert: f32, horiz: f32, rate: f32) -> Vec2 {
    let x = -horiz * rate * (rate * elapsed).sin();
    let y = vert * rate * (rate * elapsed).cos();
    vec2(x, y) * time
}

pub fn update_renderer(
    display: &Display,
    renderer: &mut ShapeRenderer,
    drone: Vec2,
    animal: Vec2,
    guess: Vec2,
) {
    renderer.clear_shapes();
    renderer.add_vertices(
        build_rectangle(drone, [0.2, 0.2], [0.0, 0.0, 1.0])
            .iter()
            .map(|x| *x),
    );
    renderer.add_vertices(
        build_rectangle(animal, [0.2, 0.2], [0.0, 1.0, 0.0])
            .iter()
            .map(|x| *x),
    );
    renderer.add_vertices(
        build_rectangle(guess, [0.2, 0.2], [1.0, 0.0, 1.0])
            .iter()
            .map(|x| *x),
    );
    renderer.update_buffer(display).unwrap();
}

pub fn save_default_config() {
    let file = File::create("default.json").expect("Failed to create file");
    let config: Config = Config::default();

    serde_json::to_writer(file, &config).unwrap();
}