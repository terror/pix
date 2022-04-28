use crate::common::*;

mod app;
mod common;
mod editor;

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  log::trace!("Initializing yew...");
  yew::start_app::<App>();
}
