use yew::{function_component, html};

#[function_component(TestDrawer)]
pub fn test_drawer() -> Html {
    html! {
      <>
        <style>
          {"
          .drawer {
            display: none;
          }
          .drawer__overlay {
            position: fixed;
            top: 0;
            right: 0;
            bottom: 0;
            left: 0;
            width: 100%;
            z-index: 200;
            opacity: 0;
            transition: opacity 0.3s;
            will-change: opacity;
            background-color: #000;
            -webkit-user-select: none;
            -moz-user-select: none;
            -ms-user-select: none;
            user-select: none;          
          }
          .drawer__header {
            padding: 1.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 1px solid #ddd;
          }
          .drawer__close {
            margin: 0;
            padding: 0;
            border: none;
            background-color: transparent;
            cursor: pointer;
            width: 15px;
            height: 15px;
            flex-shrink: 0;
            margin-left: 1rem;
          }
          .drawer__wrapper {
            position: fixed;
            top: 0;
            right: 0;
            bottom: 0;
            height: 100%;
            width: 100%;
            max-width: 500px;
            z-index: 9999;
            overflow: auto;
            transition: transform 0.3s;
            will-change: transform;
            background-color: #fff;
            display: flex;
            flex-direction: column; 
            -webkit-transform: translate3d(103%, 0, 0);
            transform: translate3d(103%, 0, 0); /* extra 3% because of box-shadow */ 
            -webkit-overflow-scrolling: touch; /* enables momentum scrolling in iOS overflow elements */
            box-shadow: 0 2px 6px #777;
          }
          .drawer--left .drawer__wrapper {
            left: 0;
            right: auto;
            -webkit-transform: translate3d(-100%, 0, 0);
            transform: translate3d(-100%, 0, 0);
          }
          .drawer.is-active {
            display: block;
          }
          .drawer.is-visible .drawer__wrapper {
            -webkit-transform: translate3d(0, 0, 0);
            transform: translate3d(0, 0, 0);
          }
          .drawer.is-visible .drawer__overlay {
            opacity: 0.5;
          }
          "}
        </style>
        <div>
          <a href="#" data-drawer-trigger="data-drawer-trigger" aria-controls="drawer-name-left" aria-expanded="false">{"Open Drawer from left side"}</a>
        </div>
        <section class="drawer drawer--left" id="drawer-name-left" data-drawer-target="data-drawer-target">
          <div class="drawer__overlay" data-drawer-close="data-drawer-close" tabindex="-1"></div>
          <div class="drawer__wrapper"></div>
        </section>
      </>
    }
}