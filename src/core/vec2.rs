#[derive(Default, Clone, Copy)]
pub struct Vec2<T> {
  x: T,
  y: T,
}

impl<T> Vec2<T> {
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  pub fn from_tuple((x, y): (T, T)) -> Self {
    Self { x, y }
  }
}

impl<T: Copy> Vec2<T> {
  pub fn x(&self) -> T {
    self.x
  }

  pub fn y(&self) -> T {
    self.y
  }
}

pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u32>;
pub type Vec2f = Vec2<f64>;

impl Vec2u {
  pub fn normalize(&self, bounds: Vec2u) -> Vec2f {
    Vec2f::new(
      self.x() as f64 / bounds.x() as f64,
      self.y() as f64 / bounds.y() as f64,
    )
  }
}
