use crate::common::*;

mod app;
mod common;
mod editor;
mod pixel;
mod position;

fn main() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  wasm_logger::init(wasm_logger::Config::default());
  log::trace!("Initializing yew...");
  yew::start_app::<App>();
}
