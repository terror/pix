use crate::common::*;

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

    let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

    match msg {
      Position(event) => {
        self
          .position
          .update(event, canvas.get_bounding_client_rect());
        true
      }
      Move(event) => {
        if event.buttons() != 1 {
          return false;
        }

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

        ctx.move_to(self.position.x, self.position.y);

        self
          .position
          .update(event, canvas.get_bounding_client_rect());

        ctx.line_to(self.position.x, self.position.y);

        ctx.stroke();

        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    use EditorMessage::*;

    html! {
      <canvas
        ref={self.node_ref.clone()}
        onresize={Callback::from(|_| ())}
        onmousedown={ctx.link().callback(Position)}
        onmouseenter={ctx.link().callback(Position)}
        onmousemove={ctx.link().callback(Move)}
      />
    }
  }
}
