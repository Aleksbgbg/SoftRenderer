#[allow(dead_code)]
mod core;
mod displays;
mod game;
mod rasterizer;

use crate::core::vec2::Vec2u;
use crate::displays::sdl2_window::Sdl2Window;
use crate::game::Game;
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

  let mut game = Game::new(BASE_SCREEN_SIZE);

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
      game.handle_event(&event);
    }

    display.clear();
    game.main_loop(&mut display);
    display.present();

    // Lock window to ~60fps for now
    thread::sleep(Duration::from_millis(1_000 / 60));
  }
}
