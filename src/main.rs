#[allow(dead_code)]
mod core;
mod displays;

use crate::core::vec2::Vec2u;
use crate::displays::display::Display;
use crate::displays::sdl2_window::Sdl2Window;
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

    game_loop(&mut display);

    display.present();
    // Lock window to ~60fps for now
    thread::sleep(Duration::from_millis(1_000 / 60));
  }
}

fn game_loop(_display: &mut dyn Display) {}
