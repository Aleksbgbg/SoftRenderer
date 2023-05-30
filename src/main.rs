#[allow(dead_code)]
mod core;
mod displays;
mod rasterizer;

use crate::core::vec2::{Vec2i, Vec2u};
use crate::core::vec4::Color;
use crate::displays::display::Display;
use crate::displays::sdl2_window::Sdl2Window;
use crate::rasterizer::triangle2d::{Fill, Triangle2d};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::Duration;

fn main() {
  let sdl_context = sdl2::init().expect("Could not initialize SDL.");

  let mut event_pump = sdl_context
    .event_pump()
    .expect("Could not obtain the SDL event pump.");

  let mut display = Sdl2Window::new(&sdl_context, Vec2u::new(1366, 768));

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

  display.draw(Triangle2d::new([Vec2i::new(500, 20); 3]), FILL);
  display.draw(
    Triangle2d::new([Vec2i::new(60, 30), Vec2i::new(70, 20), Vec2i::new(80, 30)]),
    FILL,
  );
  display.draw(
    Triangle2d::new([Vec2i::new(10, 25), Vec2i::new(20, 20), Vec2i::new(30, 80)]),
    FILL,
  );
  display.draw(
    Triangle2d::new([
      Vec2i::new(500, 200),
      Vec2i::new(200, 400),
      Vec2i::new(1300, 760),
    ]),
    FILL,
  );
  display.draw(
    Triangle2d::new([
      Vec2i::new(900, 200),
      Vec2i::new(1000, 200),
      Vec2i::new(950, 287),
    ]),
    FILL,
  );
}
