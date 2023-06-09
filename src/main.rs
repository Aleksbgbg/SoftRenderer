#[allow(dead_code)]
mod core;
mod displays;
mod rasterizer;

use crate::core::vec2::Vec2u;
use crate::core::vec4::Color;
use crate::displays::display::Display;
use crate::displays::sdl2_window::Sdl2Window;
use crate::rasterizer::triangle2d::{Fill, Triangle2d};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::Duration;

const BASE_SCREEN_SIZE: Vec2u = Vec2u::new(1366, 768);

fn main() {
  let sdl_context = sdl2::init().expect("Could not initialize SDL.");

  let mut event_pump = sdl_context
    .event_pump()
    .expect("Could not obtain the SDL event pump.");

  let mut display = Sdl2Window::new(&sdl_context, BASE_SCREEN_SIZE);

  'app_loop: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'app_loop,
        _ => {}
      }
    }

    display.clear();
    game_loop(&mut display);
    display.present();

    // Lock window to ~60fps for now
    thread::sleep(Duration::from_millis(1_000 / 60));
  }
}

fn game_loop(display: &mut dyn Display) {
  const FILL: Fill = Fill::Wireframe(Color::new(94, 129, 172, 255));

  display.draw(
    Triangle2d::new([Vec2u::new(500, 20).normalize(BASE_SCREEN_SIZE); 3]),
    FILL,
  );
  display.draw(
    Triangle2d::new([
      Vec2u::new(60, 30).normalize(BASE_SCREEN_SIZE),
      Vec2u::new(70, 20).normalize(BASE_SCREEN_SIZE),
      Vec2u::new(80, 30).normalize(BASE_SCREEN_SIZE),
    ]),
    FILL,
  );
  display.draw(
    Triangle2d::new([
      Vec2u::new(10, 25).normalize(BASE_SCREEN_SIZE),
      Vec2u::new(20, 20).normalize(BASE_SCREEN_SIZE),
      Vec2u::new(30, 80).normalize(BASE_SCREEN_SIZE),
    ]),
    FILL,
  );
  display.draw(
    Triangle2d::new([
      Vec2u::new(500, 200).normalize(BASE_SCREEN_SIZE),
      Vec2u::new(200, 400).normalize(BASE_SCREEN_SIZE),
      Vec2u::new(1300, 760).normalize(BASE_SCREEN_SIZE),
    ]),
    FILL,
  );
  display.draw(
    Triangle2d::new([
      Vec2u::new(900, 200).normalize(BASE_SCREEN_SIZE),
      Vec2u::new(1000, 200).normalize(BASE_SCREEN_SIZE),
      Vec2u::new(950, 287).normalize(BASE_SCREEN_SIZE),
    ]),
    FILL,
  );
}
