use crate::common::*;

#[derive(Debug, Default)]
pub(crate) struct Position {
  pub(crate) x: f64,
  pub(crate) y: f64,
}

impl Position {
  pub(crate) fn update(&mut self, e: MouseEvent, offset: DomRect) {
    self.x = e.client_x() as f64 - offset.left();
    self.y = e.client_y() as f64 - offset.top();
  }
}
