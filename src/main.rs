use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::thread;
use std::time::Duration;

fn main() {
  let sdl_context = sdl2::init().expect("Could not initialize SDL.");

  let mut event_pump = sdl_context
    .event_pump()
    .expect("Could not obtain the SDL event pump.");

  let video_subsystem = sdl_context
    .video()
    .expect("Could not initialize the SDL video subsystem.");
  let window = video_subsystem
    .window("SoftRenderer", 1366, 768)
    .position_centered()
    .build()
    .expect("Could not build the SDL window.");
  let mut canvas = window
    .into_canvas()
    .build()
    .expect("Could not build the window canvas.");

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

    game_loop(&mut canvas);

    canvas.present();
    // Lock window to ~60fps for now
    thread::sleep(Duration::from_millis(1_000 / 60));
  }
}

fn game_loop(_canvas: &mut Canvas<Window>) {}
