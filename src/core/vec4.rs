#[derive(Default, Clone, Copy)]
pub struct Vec4<T> {
  x: T,
  y: T,
  z: T,
  w: T,
}

impl<T> Vec4<T> {
  pub const fn new(x: T, y: T, z: T, w: T) -> Self {
    Self { x, y, z, w }
  }
}

impl<T: Copy> Vec4<T> {
  pub fn x(&self) -> T {
    self.x
  }

  pub fn y(&self) -> T {
    self.y
  }

  pub fn z(&self) -> T {
    self.z
  }

  pub fn w(&self) -> T {
    self.w
  }

  pub fn r(&self) -> T {
    self.x
  }

  pub fn g(&self) -> T {
    self.y
  }

  pub fn b(&self) -> T {
    self.z
  }

  pub fn a(&self) -> T {
    self.w
  }
}

pub type Vec4i = Vec4<i32>;
pub type Vec4u = Vec4<u32>;
pub type Vec4f = Vec4<f64>;
pub type Color = Vec4<u8>;
