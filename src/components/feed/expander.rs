use yew::{function_component, html};

#[function_component(Expander)]
pub fn expander() -> Html {
    html! {
      <div
        style="
          height: 200px; 
          width: 470px;
          min-width: 470px;
          background-color: #16202A;
          border-bottom: 1px solid #1E2832;
          position: fixed;
          border-radius: 20px 20px 0px 0px;
          bottom: 0px;
        ">
          
      </div>
    }
}
