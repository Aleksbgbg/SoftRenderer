use crate::core::vec2::{Vec2f, Vec2i, Vec2u};
use crate::core::vec4::Color;
use crate::displays::display::Display;
use std::cmp::Ordering;

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

  fn compare_to(&self, point: Point) -> Ordering {
    let value = self.eval_at(point);

    if nearly_equal(0.0, value) {
      Ordering::Equal
    } else {
      0.0_f64.total_cmp(&value)
    }
  }
}

fn sort_points(points: &mut [Point; 3], value: impl Fn(Point) -> f64) {
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

struct TriangleBoundingLine {
  equation: LinearEquation,
  expected_comparison: Ordering,
}

impl TriangleBoundingLine {
  fn new(equation: LinearEquation) -> Self {
    Self {
      equation,
      expected_comparison: Ordering::Equal,
    }
  }
}

pub struct Triangle2d {
  top_left: Point,
  bottom_right: Point,
  lines: [TriangleBoundingLine; 3],
}

impl Triangle2d {
  pub fn new(mut points: [Point; 3]) -> Self {
    sort_points(&mut points, |point| point.y());
    let top = points[0];
    let middle = points[1];
    let bottom = points[2];

    sort_points(&mut points, |point| point.x());
    let left = points[0];
    let right = points[2];

    let top_left = Point::new(left.x(), top.y());
    let bottom_right = Point::new(right.x(), bottom.y());

    let mut lines = [
      TriangleBoundingLine::new(LinearEquation::between(top, middle)),
      TriangleBoundingLine::new(LinearEquation::between(top, bottom)),
      TriangleBoundingLine::new(LinearEquation::between(middle, bottom)),
    ];

    let center = Point::new((left.x() + right.x()) / 2.0, (top.y() + bottom.y()) / 2.0);

    for line in lines.iter_mut() {
      let center_value = line.equation.eval_at(center);

      line.expected_comparison = if nearly_equal(0.0, center_value) {
        Ordering::Equal
      } else if 0.0 < center_value {
        Ordering::Less
      } else {
        Ordering::Greater
      }
    }

    Self {
      top_left,
      bottom_right,
      lines,
    }
  }

  fn top_left(&self) -> Point {
    self.top_left
  }

  fn bottom_right(&self) -> Point {
    self.bottom_right
  }

  fn point_within_bounding_lines(&self, point: Point) -> bool {
    let mut within_lines = true;

    for line in self.lines.iter() {
      let comparison = line.equation.compare_to(point);
      within_lines &= (comparison == line.expected_comparison) || (comparison == Ordering::Equal);
    }

    within_lines
  }

  fn point_on_bounding_lines(&self, point: Point) -> bool {
    let mut on_lines = false;

    for line in self.lines.iter() {
      on_lines |= line.equation.compare_to(point) == Ordering::Equal;
    }

    on_lines
  }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Fill {
  Solid(Color),
  Wireframe(Color),
}

impl dyn Display + '_ {
  pub fn draw(&mut self, triangle: Triangle2d, fill: Fill) {
    match fill {
      Fill::Solid(color) => self.draw_internal(&triangle, move |triangle, point| {
        if triangle.point_within_bounding_lines(point) {
          Some(color)
        } else {
          None
        }
      }),
      Fill::Wireframe(color) => self.draw_internal(&triangle, move |triangle, point| {
        if triangle.point_on_bounding_lines(point) {
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
