use yew::{function_component, html, Properties, Html};

#[derive(Properties, PartialEq)]
pub struct CenterFormWrapperProps {
    pub inner_html: Html,
}

#[function_component(CenterFormWrapper)]
pub fn center_form_wrapper(props: &CenterFormWrapperProps) -> Html {
    html! {
      <div
        style="
          width: 100%;
          height: 100vh;
          display: flex;
          justify-content: center;
          align-items: center;
        "
      >
        {props.inner_html.clone()}
      </div>
    }
}