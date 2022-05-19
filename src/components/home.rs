use crate::components::feed::expander::component::Expander;
use crate::components::feed::posts_list::PostsList;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use yew::{function_component, html, use_state, Callback};

#[function_component(Home)]
pub fn home() -> Html {
    let show_expander = use_state(|| false);
    let show_expander_cloned = show_expander.clone();
    let show_expander_cloned_second = show_expander.clone();
    let change_show_expander = Callback::from(move |_| {
        show_expander_cloned.set(!*show_expander_cloned);
    });
    let inner_html = html!{<ExpandMoreContent/>};
    html! {
      <div
        style="
        padding-top: 43px;
      "
      >
        <PostsList callback={change_show_expander} is_expander_opened={*show_expander_cloned_second.clone()}/>
        if *show_expander {
          <Expander inner_html={inner_html}/>
        }
      </div>
    }
}
