use yew::{function_component, html};

#[function_component(AntdBtn)]
pub fn antd_btn() -> Html {
    html! {
      <button type="button" class="ant-btn">
        <span>{"Default"}</span>
      </button>
    }
}
