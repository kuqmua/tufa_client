use crate::components::svg::expand_more::ExpandMore;
use crate::components::svg::favorite::Favorite;
use crate::components::svg::favorite_border::FavoriteBorder;
use crate::components::svg::share::Share;
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
                display: flex;
                flex-direction: row;
                justify-content: flex-start;
                align-items: flex-start;
              "
          >
            <img
              src="https://telegrator.ru/wp-content/uploads/2021/05/chat_avatar-136.jpg"
              alt="avatar"
              style="
                  width: 20px;
                  height: 20px;
                  border-radius: 2px;
                  margin-right: 8px;
                "
            />
            <div
              style="
                  color: white;
                  margin-bottom: 8px;
                  font-size: 20px;
                "
            >
              {"author r/subreddit 1h"}
            </div>
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
            <ExpandMore height={"24px".to_owned()} width={"24px".to_owned()} fill={"#A2B0B9".to_owned()}/>
          </div>
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
            <Share height={"24px".to_owned()} width={"24px".to_owned()} fill={"#A2B0B9".to_owned()}/>
          </div>
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
            <Favorite height={"24px".to_owned()} width={"24px".to_owned()} fill={"#A2B0B9".to_owned()}/>
          </div>
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
            <FavoriteBorder height={"24px".to_owned()} width={"24px".to_owned()} fill={"#A2B0B9".to_owned()}/>
          </div>
        </div>
      </div>
    }
}
