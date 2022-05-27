use yew::{function_component, html};

#[function_component(TestDrawerButton)]
pub fn test_drawer_button() -> Html {
    html! {
      <>
        <div>
          <a href="#" data-drawer-trigger="data-drawer-trigger" aria-controls="drawer-name-left" aria-expanded="false">{"Open Drawer from left side"}</a>
        </div>
      </>
    }
}