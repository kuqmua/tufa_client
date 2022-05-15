use crate::components::feed::expand_more_button::ExpandMoreButton;
use crate::components::feed::favorite_button::FavoriteButton;
use crate::components::feed::share_button::ShareButton;
use yew::prelude::*;

#[function_component(Post)]
pub fn post() -> Html {
    html! {
      <div
        style="
            display: flex;
            flex-direction: row;
            justify-content: center;
            align-items: flex-start;
            margin-top: 8px;
            margin-bottom: 8px;
            background-color: #1E2832;
          "
      >
        <div
          style="
              width: 60px;
              margin-right: 8px;
              display: flex;
              justify-content: flex-start;
            "
        >
          <img
            src="https://telegrator.ru/wp-content/uploads/2021/05/chat_avatar-136.jpg"
            alt="avatar"
            style="
                width: 60px;
                height: 60px;
                border-radius: 10px;
              "
           />
        </div>
        <div
          style="
              display: flex;
              flex-direction: column;
            "
        >
          <div
            style="
              color: white;
              margin-bottom: 8px;
              font-size: 20px;
            "
          >
            {"Author"}
          </div>
          <div
            style="
                color: white;
                margin-bottom: 8px;
                font-size: 18px;
              "
          >
            {"
                some huge text some huge text some huge text
                some huge text some huge text some huge text
                some huge text some huge text some huge text
                some huge text some huge text some huge text
                some huge text some huge text some huge text
                some huge text some huge text some huge text
              "}
          </div>
          <img
              src="https://funik.ru/wp-content/uploads/2018/10/17478da42271207e1d86.jpg"
              alt="avatar"
              style="
                  width: 100%;
                  border-radius: 15px;
                "
          />
        </div>
        <div
          style="
              display: flex;
              flex-direction: column;
              justify-content: flex-start;
              margin-left: 8px;
              width: 50px;
            "
        >
          <ExpandMoreButton/>
          <ShareButton/>
          <FavoriteButton/>
        </div>
      </div>
    }
}
