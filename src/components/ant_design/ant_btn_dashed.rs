use yew::{function_component, html};

#[function_component(AntBtnDashed)]
pub fn ant_btn_dashed() -> Html {
    html! {
      <button type="button" class="ant-btn ant-btn-dashed">
        <span>{"Dashed"}</span>
      </button>
    }
}