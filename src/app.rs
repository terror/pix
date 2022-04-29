use crate::common::*;

#[function_component(App)]
pub(crate) fn app() -> Html {
  html! {
    <>
      <p>{ "pix" }</p>
      <Editor/>
    </>
  }
}
