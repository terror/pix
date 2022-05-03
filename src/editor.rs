use crate::common::*;

#[derive(Debug)]
pub(crate) struct Editor {
  canvas: NodeRef,
  position: Position,
  settings: EditorSettings,
}

#[derive(Debug)]
pub(crate) enum EditorMessage {
  Color(Event),
  Move(MouseEvent),
  Position(MouseEvent),
}

#[derive(Debug)]
pub(crate) struct EditorSettings {
  pixel_width: u32,
  pixel_height: u32,
  canvas_width: u32,
  canvas_height: u32,
  pixel_color: String,
}

impl Default for EditorSettings {
  fn default() -> Self {
    Self {
      pixel_width: 32,
      pixel_height: 32,
      canvas_width: 800,
      canvas_height: 640,
      pixel_color: String::from("#000000"),
    }
  }
}

impl Component for Editor {
  type Message = EditorMessage;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      canvas: NodeRef::default(),
      position: Position::default(),
      settings: EditorSettings::default(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();

    let rect = canvas.get_bounding_client_rect();

    match msg {
      EditorMessage::Position(_) => {
        let ctx = canvas
          .get_context("2d")
          .unwrap()
          .unwrap()
          .dyn_into::<web_sys::CanvasRenderingContext2d>()
          .unwrap();

        ctx.set_fill_style(&JsValue::from_str(&self.settings.pixel_color));

        ctx.fill_rect(
          self.position.x as f64,
          self.position.y as f64,
          self.settings.pixel_width as f64,
          self.settings.pixel_height as f64,
        );

        true
      }
      EditorMessage::Move(event) => {
        self.position.update(
          event,
          rect,
          self.settings.pixel_width as f64,
          self.settings.pixel_height as f64,
        );
        true
      }
      EditorMessage::Color(event) => {
        self.settings.pixel_color = event
          .target()
          .unwrap()
          .unchecked_into::<HtmlInputElement>()
          .value();
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class={classes!("container")}>
        <canvas
          ref={self.canvas.clone()}
          onmousedown={ctx.link().callback(EditorMessage::Position)}
          onmousemove={ctx.link().callback(EditorMessage::Move)}
        />
        <div class={classes!("settings")}>
          <input onchange={ctx.link().callback(EditorMessage::Color)} type="color"/>
        </div>
      </div>
    }
  }

  fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
    if first_render {
      let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();

      canvas.set_width(self.settings.canvas_width);
      canvas.set_height(self.settings.canvas_height);

      let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

      ctx.begin_path();

      ctx.set_stroke_style(&JsValue::from_str("rgba(150, 150, 150, 0.75)"));

      let mut x = 0;
      let mut y = 0;

      while x <= canvas.width() {
        ctx.move_to(x as f64, 0.0);
        ctx.line_to(x as f64, canvas.height() as f64);
        x += self.settings.pixel_width;
      }

      while y <= canvas.height() {
        ctx.move_to(0.0, y as f64);
        ctx.line_to(canvas.width() as f64, y as f64);
        y += self.settings.pixel_height;
      }

      ctx.stroke();
    }
  }
}
