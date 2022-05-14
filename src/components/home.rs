use crate::{store::YewduxStore};
use yew::prelude::*;
use yewdux::prelude::DispatchProps;
use yewdux::prelude::PersistentStore;

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = DispatchProps<PersistentStore<YewduxStore>>;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    fn destroy(&mut self, _ctx: &Context<Self>) {}
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div
            style="
              display: flex;
              flex-direction: row;
              justify-content: center;
              align-items: flex-start;
              margin-top: 5px;
              margin-bottom: 5px;
              margin-left: 5px;
              margin-right: 5px;
              width: 500px;
              background-color: #1E2832;
            "
          >
            <div
              style="
                width: 60px;
                margin-right: 5px;
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
                    margin-right: 5px;
                  "
                />
                <div
                  style="
                    color: #A2B0B9;
                  "
                >
                  {"author r/subreddit 1h"}
                </div>
              </div>
              <div
                style="
                  color: #A2B0B9;
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
                    border-radius: 10px;
                  "
              />
            </div>
            <div
              style="
                display: flex;
                flex-direction: column;
                justify-content: flex-start;
                margin-left: 5px;
                width: 50px;
              "
            >
              <div
                style="
                  border-radius: 10px;
                  border: 2px solid #A2B0B9;
                  width: 35px;
                  height: 35px;
                  margin-bottom: 10px;
                  display: flex;
                  justify-content: center;
                  align-items: center;
                "
              >
                <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#A2B0B9">
                  <path d="M0 0h24v24H0z" fill="none"/>
                  <path d="M18 16.08c-.76 0-1.44.3-1.96.77L8.91 12.7c.05-.23.09-.46.09-.7s-.04-.47-.09-.7l7.05-4.11c.54.5 1.25.81 2.04.81 1.66 0 3-1.34 3-3s-1.34-3-3-3-3 1.34-3 3c0 .24.04.47.09.7L8.04 9.81C7.5 9.31 6.79 9 6 9c-1.66 0-3 1.34-3 3s1.34 3 3 3c.79 0 1.5-.31 2.04-.81l7.12 4.16c-.05.21-.08.43-.08.65 0 1.61 1.31 2.92 2.92 2.92 1.61 0 2.92-1.31 2.92-2.92s-1.31-2.92-2.92-2.92z"/>
                </svg>
              </div>
              <div
                style="
                  border-radius: 10px;
                  border: 2px solid #A2B0B9;
                  width: 35px;
                  height: 35px;
                  margin-bottom: 10px;
                  display: flex;
                  justify-content: center;
                  align-items: center;
                "
              >
                <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#A2B0B9">
                  <path d="M0 0h24v24H0z" fill="none"/>
                  <path d="M18 16.08c-.76 0-1.44.3-1.96.77L8.91 12.7c.05-.23.09-.46.09-.7s-.04-.47-.09-.7l7.05-4.11c.54.5 1.25.81 2.04.81 1.66 0 3-1.34 3-3s-1.34-3-3-3-3 1.34-3 3c0 .24.04.47.09.7L8.04 9.81C7.5 9.31 6.79 9 6 9c-1.66 0-3 1.34-3 3s1.34 3 3 3c.79 0 1.5-.31 2.04-.81l7.12 4.16c-.05.21-.08.43-.08.65 0 1.61 1.31 2.92 2.92 2.92 1.61 0 2.92-1.31 2.92-2.92s-1.31-2.92-2.92-2.92z"/>
                </svg>
              </div>
            </div>
          </div>
        }
    }
}
