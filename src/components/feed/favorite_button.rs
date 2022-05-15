use crate::components::svg::favorite::Favorite;
use crate::components::svg::favorite_border::FavoriteBorder;
use yew::{function_component, html, use_state, Callback};

#[function_component(FavoriteButton)]
pub fn favorite_button() -> Html {
    let is_liked = use_state(|| false);
    let is_liked_cloned = is_liked.clone();
    let change_is_liked = Callback::from(move |_| {
        is_liked_cloned.set(!*is_liked_cloned);
    });
    html! {
      <button
        style="
          border-radius: 10px;
          border: 1px solid #A2B0B9;
          width: 35px;
          height: 35px;
          margin-bottom: 8px;
          display: flex;
          justify-content: center;
          align-items: center;
          background-color: #1E2832;
        "
        onclick={change_is_liked}
      >
        if *is_liked {
          <Favorite height={"24px".to_owned()} width={"24px".to_owned()} fill={"#ffffa2".to_owned()}/>
        }
        else {
          <FavoriteBorder height={"24px".to_owned()} width={"24px".to_owned()} fill={"#A2B0B9".to_owned()}/>
        }
      </button>
    }
}
