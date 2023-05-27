use crate::core::vec2::{Vec2i, Vec2u};
use crate::core::vec4::Color;

pub trait Display {
  fn size(&self) -> Vec2u;
  fn set_size(&mut self, size: Vec2u);

  fn draw_pixel(&mut self, position: Vec2i, color: Color);
}
