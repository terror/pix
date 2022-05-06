use crate::common::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Position {
  pub(crate) x: f64,
  pub(crate) y: f64,
}

impl Position {
  pub(crate) fn update(
    &mut self,
    e: MouseEvent,
    offset: DomRect,
    pixel_width: f64,
    pixel_height: f64,
  ) {
    self.x = ((e.client_x() as f64 - offset.left() - (pixel_width / 2.0))
      / pixel_width)
      .round()
      * pixel_width;
    self.y = ((e.client_y() as f64 - offset.top() - (pixel_height / 2.0))
      / pixel_height)
      .round()
      * pixel_height;
  }
}
