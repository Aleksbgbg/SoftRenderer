use crate::core::vec2::{Vec2i, Vec2u};
use crate::core::vec4::Color;
use crate::displays::display::Display;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::{pixels, Sdl, VideoSubsystem};

pub struct Sdl2Window {
  _video_subsystem: VideoSubsystem,
  canvas: WindowCanvas,
}

impl Sdl2Window {
  pub fn new(sdl_context: &Sdl, size: Vec2u) -> Self {
    let video_subsystem = sdl_context
      .video()
      .expect("Could not initialize the SDL video subsystem.");
    let window = video_subsystem
      .window("SoftRenderer", size.x(), size.y())
      .position_centered()
      .build()
      .expect("Could not build the SDL window.");
    let canvas = window
      .into_canvas()
      .build()
      .expect("Could not build the window canvas.");

    Sdl2Window {
      _video_subsystem: video_subsystem,
      canvas,
    }
  }

  pub fn clear(&mut self) {
    self.canvas.set_draw_color(pixels::Color::BLACK);
    self.canvas.clear();
  }

  pub fn present(&mut self) {
    self.canvas.present();
  }
}

impl Display for Sdl2Window {
  fn size(&self) -> Vec2u {
    Vec2u::from_tuple(self.canvas.window().size())
  }

  fn set_size(&mut self, size: Vec2u) {
    self
      .canvas
      .window_mut()
      .set_size(size.x(), size.y())
      .expect("Could not resize SDL window.");
  }

  fn draw_pixel(&mut self, position: Vec2i, color: Color) {
    self.canvas.set_draw_color(pixels::Color::RGBA(
      color.r(),
      color.g(),
      color.b(),
      color.a(),
    ));
    self
      .canvas
      .draw_point(Point::new(position.x(), position.y()))
      .expect("Failed to draw point.");
  }
}
