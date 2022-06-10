use yew::{function_component, html};

#[function_component(AntdBtnPrimary)]
pub fn antd_btn_primary() -> Html {
    html! {
      <button type="button" class="ant-btn ant-btn-primary">
        <span>{"Primary"}</span>
      </button>
    }
}
