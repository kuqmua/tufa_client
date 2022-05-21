use yew::{function_component, html};

#[function_component(Drawer)]
pub fn drawer() -> Html {
    html! {
      <>
      <input type="checkbox" id="menu-opener"/>
      <aside class="DrawerMenu" id="menu">
          <nav class="Menu"></nav>
          <label for="menu-opener" style="background-color: black; opacity: 0.1;"></label>
      </aside>
      </>
    }
}
