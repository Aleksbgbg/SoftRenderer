use crate::core::vec2::{Vec2f, Vec2i, Vec2u};
use crate::core::vec4::Color;
use crate::displays::display::Display;

// Ideally this should be calculated to be
// line_width_pixels / min(screen_width, screen_height)
// which is the desired line width normalized to the smallest screen dimension
// (to ensure lines would definitely be drawn). Alternatively max can be used if
// thin lines are a requirement.
const LINE_WIDTH: f64 = 0.001;

pub type Point = Vec2f;

fn nearly_equal(left: f64, right: f64) -> bool {
  const TOLERANCE: f64 = LINE_WIDTH;
  (left - right).abs() < TOLERANCE
}

struct LinearEquation {
  m: f64,
  c: f64,
}

impl LinearEquation {
  fn new(m: f64, c: f64) -> Self {
    Self { m, c }
  }

  fn between(a: Point, b: Point) -> Self {
    let m = if a.x() == b.x() {
      0.0
    } else {
      (a.y() - b.y()) / (a.x() - b.x())
    };
    let c = a.y() - (m * a.x());

    Self::new(m, c)
  }

  fn eval_at(&self, point: Point) -> f64 {
    (self.m * point.x()) + self.c - point.y()
  }

  fn eq(&self, point: Point) -> bool {
    nearly_equal(0.0, self.eval_at(point))
  }

  fn le(&self, point: Point) -> bool {
    0.0 <= self.eval_at(point)
  }

  fn ge(&self, point: Point) -> bool {
    0.0 >= self.eval_at(point)
  }
}

pub type TrianglePoints = [Point; 3];

fn sort_points(points: &mut TrianglePoints, value: impl Fn(Point) -> f64) {
  if value(points[0]) > value(points[1]) {
    points.swap(0, 1);
  }
  if value(points[1]) > value(points[2]) {
    points.swap(1, 2);
  }

  if value(points[0]) > value(points[1]) {
    points.swap(0, 1);
  }
}

pub struct Triangle2d {
  top_left: Point,
  bottom_right: Point,
  left_line: LinearEquation,
  right_line: LinearEquation,
  bottom_line: LinearEquation,
}

impl Triangle2d {
  pub fn new(mut points: TrianglePoints) -> Self {
    sort_points(&mut points, |point| point.y());
    let top = points[0];
    let middle = points[1];
    let bottom = points[2];

    sort_points(&mut points, |point| point.x());
    let left = points[0];
    let right = points[2];

    let top_left = Point::new(left.x(), top.y());
    let bottom_right = Point::new(right.x(), bottom.y());

    let left_line = LinearEquation::between(top, middle);
    let right_line = LinearEquation::between(top, bottom);
    let bottom_line = LinearEquation::between(middle, bottom);

    Self {
      top_left,
      bottom_right,
      left_line,
      right_line,
      bottom_line,
    }
  }

  fn top_left(&self) -> Point {
    self.top_left
  }

  fn bottom_right(&self) -> Point {
    self.bottom_right
  }

  fn left_line(&self) -> &LinearEquation {
    &self.left_line
  }

  fn right_line(&self) -> &LinearEquation {
    &self.right_line
  }

  fn bottom_line(&self) -> &LinearEquation {
    &self.bottom_line
  }
}

#[allow(dead_code)]
pub enum Fill {
  Solid(Color),
  Wireframe(Color),
}

impl dyn Display + '_ {
  pub fn draw(&mut self, triangle: Triangle2d, fill: Fill) {
    match fill {
      Fill::Solid(color) => self.draw_internal(&triangle, move |triangle, point| {
        if triangle.left_line().ge(point)
          && triangle.right_line().ge(point)
          && triangle.bottom_line().le(point)
        {
          Some(color)
        } else {
          None
        }
      }),
      Fill::Wireframe(color) => self.draw_internal(&triangle, move |triangle, point| {
        if triangle.left_line().eq(point)
          || triangle.right_line().eq(point)
          || triangle.bottom_line().eq(point)
        {
          Some(color)
        } else {
          None
        }
      }),
    };
  }

  fn draw_internal(
    &mut self,
    triangle: &Triangle2d,
    fill_at: impl Fn(&Triangle2d, Point) -> Option<Color>,
  ) {
    let screen_size = self.size();

    let top = (triangle.top_left().y() * screen_size.y() as f64) as u32;
    let bottom = (triangle.bottom_right().y() * screen_size.y() as f64) as u32;
    let left = (triangle.top_left().x() * screen_size.x() as f64) as u32;
    let right = (triangle.bottom_right().x() * screen_size.x() as f64) as u32;

    for y in top..=bottom {
      for x in left..=right {
        if let Some(color) = fill_at(triangle, Vec2u::new(x, y).normalize(screen_size)) {
          self.draw_pixel(Vec2i::new(x as i32, y as i32), color);
        }
      }
    }
  }
}
