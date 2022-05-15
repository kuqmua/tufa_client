use crate::components::svg::favorite::Favorite;
use yew::{function_component, html, use_state};

#[function_component(FavoriteButton)]
pub fn favorite_button() -> Html {
    let is_liked = use_state(|| false);
    html! {
      <div
        style="
          border-radius: 10px;
          border: 1px solid #A2B0B9;
          width: 35px;
          height: 35px;
          margin-bottom: 8px;
          display: flex;
          justify-content: center;
          align-items: center;
        "
      >
        if *is_liked {
          <Favorite height={"24px".to_owned()} width={"24px".to_owned()} fill={"#ffffa2".to_owned()}/>
        }
        else {
        //   <FavoriteBorder height={"24px".to_owned()} width={"24px".to_owned()} fill={"#A2B0B9".to_owned()}/>
        <Favorite height={"24px".to_owned()} width={"24px".to_owned()} fill={"#ffffa2".to_owned()}/>
        }
      </div>
    }
}
