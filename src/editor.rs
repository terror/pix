use crate::common::*;

#[derive(Debug)]
pub(crate) struct Editor {
  canvas: NodeRef,
  pixels: Vec<(Position, String)>,
  redo: Vec<(Position, String)>,
  position: Position,
  settings: EditorSettings,
}

#[derive(Debug)]
pub(crate) enum EditorMessage {
  ChangeColor(Event),
  Clear(MouseEvent),
  Context(MouseEvent),
  Draw(MouseEvent),
  Move(MouseEvent),
  Redo(MouseEvent),
  Undo(MouseEvent),
}

#[derive(Debug)]
pub(crate) struct EditorSettings {
  canvas_height: u32,
  canvas_width: u32,
  pixel_color: String,
  pixel_height: u32,
  pixel_width: u32,
}

impl Default for EditorSettings {
  fn default() -> Self {
    Self {
      canvas_height: 640,
      canvas_width: 800,
      pixel_color: String::from("#000000"),
      pixel_height: 32,
      pixel_width: 32,
    }
  }
}

impl Component for Editor {
  type Message = EditorMessage;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      canvas: NodeRef::default(),
      pixels: Vec::new(),
      position: Position::default(),
      redo: Vec::new(),
      settings: EditorSettings::default(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
    match message {
      ChangeColor(event) => self.change_pixel_color(event),
      Clear(_) => self.clear(),
      Context(event) => self.clear_pixel(event),
      Draw(_) => self.draw_pixel(),
      Move(event) => self.update_position(event),
      Redo(_) => self.redo(),
      Undo(_) => self.undo(),
    }
    .is_ok()
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class={classes!("container")}>
        <canvas
          ref={self.canvas.clone()}
          onmousedown={ctx.link().callback(Draw)}
          onmousemove={ctx.link().callback(Move)}
          oncontextmenu={ctx.link().callback(Context)}
        />
        <div class={classes!("settings")}>
          <input onchange={ctx.link().callback(ChangeColor)} type="color"/>
          <button class={classes!("button")} onclick={ctx.link().callback(Clear)}>
            <i class={classes!(vec!["fa", "fa-trash"])}></i>
          </button>
          <button class={classes!("button")} hidden={self.pixels.len() == 0} onclick={ctx.link().callback(Undo)}>
            <i class={classes!(vec!["fa", "fa-rotate-left"])}></i>
          </button>
          <button class={classes!("button")} hidden={self.redo.len() == 0} onclick={ctx.link().callback(Redo)}>
            <i class={classes!(vec!["fa", "fa-rotate-right"])}></i>
          </button>
        </div>
      </div>
    }
  }

  fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
    if first_render {
      if let Err(error) = self.draw_grid() {
        log::error!("{error}");
      }
    }
  }
}

impl Editor {
  fn canvas(&self) -> Result<HtmlCanvasElement> {
    Ok(
      self
        .canvas
        .cast::<HtmlCanvasElement>()
        .ok_or("Unable to fetch canvas")?,
    )
  }

  fn context(
    &self,
    canvas: HtmlCanvasElement,
  ) -> Result<web_sys::CanvasRenderingContext2d> {
    Ok(
      canvas
        .get_context("2d")
        .unwrap()
        .ok_or("Unable to fetch canvas context")?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap(),
    )
  }

  fn change_pixel_color(&mut self, event: Event) -> Result {
    self.settings.pixel_color = event
      .target()
      .ok_or("Failed to grab target from event")?
      .unchecked_into::<HtmlInputElement>()
      .value();

    Ok(())
  }

  fn clear(&mut self) -> Result {
    self.pixels.clear();
    self.draw_grid()
  }

  fn clear_pixel(&mut self, event: MouseEvent) -> Result {
    event.prevent_default();

    self.pixels = self
      .pixels
      .iter()
      .filter(|(pixel, _)| {
        !(pixel.x == self.position.x && pixel.y == self.position.y)
      })
      .cloned()
      .collect();

    self.draw()
  }

  fn draw_grid(&self) -> Result {
    let canvas = self.canvas()?;

    canvas.set_width(self.settings.canvas_width);
    canvas.set_height(self.settings.canvas_height);

    let ctx = self.context(canvas.clone())?;

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

    Ok(())
  }

  fn draw_pixel(&mut self) -> Result {
    self
      .pixels
      .push((self.position.clone(), self.settings.pixel_color.clone()));
    self.draw()
  }

  fn draw(&self) -> Result {
    let context = self.context(self.canvas()?)?;

    self.draw_grid()?;

    self.pixels.iter().for_each(|(pixel, color)| {
      context.set_fill_style(&JsValue::from_str(color));
      context.fill_rect(
        pixel.x as f64,
        pixel.y as f64,
        self.settings.pixel_width as f64,
        self.settings.pixel_height as f64,
      );
    });

    Ok(())
  }

  fn update_position(&mut self, event: MouseEvent) -> Result {
    self.position.update(
      event,
      self.canvas()?.get_bounding_client_rect(),
      self.settings.pixel_width as f64,
      self.settings.pixel_height as f64,
    );

    Ok(())
  }

  fn undo(&mut self) -> Result {
    self
      .redo
      .push(self.pixels.pop().ok_or("No pixels to undo")?);
    self.draw()
  }

  fn redo(&mut self) -> Result {
    self
      .pixels
      .push(self.redo.pop().ok_or("No pixels to redo")?);
    self.draw()
  }
}
