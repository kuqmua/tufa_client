use yew::{function_component, html};

#[function_component(AntBtn)]
pub fn antd_btn() -> Html {
    html! {
      <button type="button" class="ant-btn">
        <span>{"Default"}</span>
      </button>
    }
}
