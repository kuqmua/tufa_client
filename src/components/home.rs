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
              align-items: center;
              margin-left: 5px;
              margin-right: 5px;
              width: 300px;
            "
          >
            <img 
              src="https://telegrator.ru/wp-content/uploads/2021/05/chat_avatar-136.jpg" 
              alt="avatar"
              style="
                width: 50px;
                height: 50px;
                border-radius: 5px;
                margin-right: 5px;
              "
            />
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
                  border-radius: 5px;
                  border: 2px solid #A2B0B9;
                  width: 50px;
                  height: 50px;
                  margin-bottom: 10px;
                "
              >
              </div>
              <div
                style="
                  border-radius: 5px;
                  border: 2px solid #A2B0B9;
                  width: 50px;
                  height: 50px;
                  margin-bottom: 10px;
                "
              >
              </div>
              <div
                style="
                  border-radius: 5px;
                  border: 2px solid #A2B0B9;
                  width: 50px;
                  height: 50px;
                  margin-bottom: 10px;
                "
              >
              </div>
              <div
                style="
                  border-radius: 5px;
                  border: 2px solid #A2B0B9;
                  width: 50px;
                  height: 50px;
                  margin-bottom: 10px;
                "
              >
              </div>
              <div
                style="
                  border-radius: 5px;
                  border: 2px solid #A2B0B9;
                  width: 50px;
                  height: 50px;
                  margin-bottom: 10px;
                "
              >
              </div>
            </div>
          </div>
        }
    }
}
