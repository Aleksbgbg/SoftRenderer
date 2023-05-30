use crate::core::vec2::Point;
use crate::core::vec4::Color;
use crate::displays::display::Display;

fn nearly_equal(left: f64, right: f64) -> bool {
  const TOLERANCE: f64 = 1.0;

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
      (a.y() - b.y()) as f64 / (a.x() - b.x()) as f64
    };
    let c = a.y() as f64 - (m * a.x() as f64);

    Self::new(m, c)
  }

  fn eval_at(&self, point: Point) -> f64 {
    let x = point.x() as f64;
    let y = point.y() as f64;

    (self.m * x) + self.c - y
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

fn sort_points(points: &mut TrianglePoints, value: impl Fn(Point) -> i32) {
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
    for y in triangle.top_left().y()..=triangle.bottom_right().y() {
      for x in triangle.top_left().x()..=triangle.bottom_right().x() {
        let point = Point::new(x, y);
        if let Some(color) = fill_at(triangle, point) {
          self.draw_pixel(point, color);
        }
      }
    }
  }
}
