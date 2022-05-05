pub(crate) use std::panic;

pub(crate) use {
  wasm_bindgen::{JsCast, JsValue},
  web_sys::{DomRect, HtmlCanvasElement, HtmlInputElement},
  yew::{prelude::*, NodeRef},
};

pub(crate) use crate::{
  app::App,
  editor::{Editor, EditorMessage::*},
  position::Position,
};

pub(crate) type Error = Box<dyn std::error::Error>;
pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;
