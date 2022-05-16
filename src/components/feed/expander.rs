use yew::{function_component, html};

#[function_component(Expander)]
pub fn expander() -> Html {
    html! {
      <div
        style="
          height: 400px; 
          width: 486px;
          min-width: 470px;
          background-color: #16202A;
          border-top: 1px solid white;
          border-left: 1px solid white;
          border-right: 1px solid white;
          position: fixed;
          border-radius: 30px 30px 0px 0px;
          bottom: 0px;
        ">
          
      </div>
    }
}
