mod math;
mod mesh;
mod graphics;
mod engine;

use engine::Engine;
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;
const WIDTH: usize=800;
const HEIGHT: usize=600;
fn main() {
    let mut window = Window::new(
        "Rasterizer-CPU",
        WIDTH, HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);
    let mut engine = Engine::new(WIDTH, HEIGHT);
    let mut last_frame_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f32();
        last_frame_time=now;

        engine.update(delta_time);
        engine.render();
        window.update_with_buffer(&engine.framebuffer.color_buffer, WIDTH, HEIGHT).unwrap();
    }
}