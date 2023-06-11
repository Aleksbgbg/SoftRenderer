use crate::core::vec2::Vec2u;
use crate::core::vec4::Color;
use crate::displays::display::Display;
use crate::rasterizer::triangle2d::{Fill, Triangle2d};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Game {
  base_screen_size: Vec2u,
  solid_fill: bool,
}

impl Game {
  pub fn new(base_screen_size: Vec2u) -> Self {
    Self {
      base_screen_size,
      solid_fill: false,
    }
  }

  pub fn handle_event(&mut self, event: &Event) {
    if let Event::KeyDown {
      keycode: Some(Keycode::F1),
      ..
    } = event
    {
      self.solid_fill = !self.solid_fill;
    }
  }

  pub fn main_loop(&mut self, display: &mut dyn Display) {
    let fill = if self.solid_fill {
      Fill::Solid(Color::new(94, 129, 172, 255))
    } else {
      Fill::Wireframe(Color::new(94, 129, 172, 255))
    };

    display.draw(
      Triangle2d::new([Vec2u::new(500, 20).normalize(self.base_screen_size); 3]),
      fill,
    );
    display.draw(
      Triangle2d::new([
        Vec2u::new(60, 30).normalize(self.base_screen_size),
        Vec2u::new(70, 20).normalize(self.base_screen_size),
        Vec2u::new(80, 30).normalize(self.base_screen_size),
      ]),
      fill,
    );
    display.draw(
      Triangle2d::new([
        Vec2u::new(10, 25).normalize(self.base_screen_size),
        Vec2u::new(20, 20).normalize(self.base_screen_size),
        Vec2u::new(30, 80).normalize(self.base_screen_size),
      ]),
      fill,
    );
    display.draw(
      Triangle2d::new([
        Vec2u::new(500, 200).normalize(self.base_screen_size),
        Vec2u::new(200, 400).normalize(self.base_screen_size),
        Vec2u::new(1300, 760).normalize(self.base_screen_size),
      ]),
      fill,
    );
    display.draw(
      Triangle2d::new([
        Vec2u::new(900, 200).normalize(self.base_screen_size),
        Vec2u::new(1000, 200).normalize(self.base_screen_size),
        Vec2u::new(950, 287).normalize(self.base_screen_size),
      ]),
      fill,
    );
  }
}
