use crate::common::*;

#[derive(Debug, Default)]
pub(crate) struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn update(&mut self, e: MouseEvent) {
    self.x = e.client_x();
    self.y = e.client_y();
  }
}

pub(crate) enum EditorMessage {
  Position(MouseEvent),
  Move(MouseEvent),
}

pub(crate) struct Editor {
  node_ref: NodeRef,
  position: Position,
}

impl Component for Editor {
  type Message = EditorMessage;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      node_ref: NodeRef::default(),
      position: Position::default(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    use EditorMessage::*;

    match msg {
      Position(e) => {
        self.position.update(e);
        true
      }
      Move(e) if e.buttons() == 1 => {
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let ctx = canvas
          .get_context("2d")
          .unwrap()
          .unwrap()
          .dyn_into::<web_sys::CanvasRenderingContext2d>()
          .unwrap();

        ctx.begin_path();

        ctx.set_line_width(5.0);
        ctx.set_line_cap("round");
        ctx.set_stroke_style(&JsValue::from_str("#c0392b"));

        ctx.move_to(self.position.x as f64, self.position.y as f64);
        self.position.update(e);
        ctx.line_to(self.position.x as f64, self.position.y as f64);

        ctx.stroke();

        true
      }
      Move(_) => false,
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    use EditorMessage::*;

    html! {
      <canvas
        ref={self.node_ref.clone()}
        onresize={Callback::from(|_| ())}
        onmousedown={ctx.link().callback(move |e: MouseEvent| Position(e))}
        onmouseenter={ctx.link().callback(move |e: MouseEvent| Position(e))}
        onmousemove={ctx.link().callback(move |e: MouseEvent| Move(e))}
      />
    }
  }

  fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
    let _has_attributes =
      self.node_ref.cast::<Element>().unwrap().has_attributes();
  }
}
