use yew::{function_component, html, Properties};
use colorsys::Hsl;

#[derive(Debug, PartialEq, Clone)]
pub struct BadgeOffset {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BadgeStatus {
    Success,
    Processing,
    Default,
    Error,
    Warning,
}

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub color: Option<Hsl>,// Customize Badge dot color	string	-	3.16.0
    pub count: Option<i64>,//	Number to show in badge	ReactNode		
    pub dot: Option<bool>,// Whether to display a red dot instead of count	boolean	false	
    pub offset: Option<BadgeOffset>,//	set offset of the badge dot, like[x, y]	[number, number]	-	
    // pub overflow_count: Option<i64>,//dont think it would be usefull//	Max count to show	number	99	
    pub show_zero: Option<bool>,//	Whether to show badge when count is zero	boolean	false	
    pub status: Option<BadgeStatus>,//	Set Badge as a status dot	success | processing | default | error | warning	''	
    pub text: Option<String>,//	If status is set, text sets the display text of the status dot	string	''	
    pub title: Option<String>,//	Text to show when hovering over the badge	string	count
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
  let sup = match props.count {
    None => html!{},
    Some(count) => {
      let count_to_show: String;
      if count > 99 {
        count_to_show = String::from("99");
      }
      else {
        count_to_show = count.to_string();
      }
      let numbers = html!{
        <>
          <p class="ant-scroll-number-only-unit">{"0"}</p>
          <p class="ant-scroll-number-only-unit">{"1"}</p>
          <p class="ant-scroll-number-only-unit">{"2"}</p>
          <p class="ant-scroll-number-only-unit">{"3"}</p>
          <p class="ant-scroll-number-only-unit">{"4"}</p>
          <p class="ant-scroll-number-only-unit">{"5"}</p>
          <p class="ant-scroll-number-only-unit">{"6"}</p>
          <p class="ant-scroll-number-only-unit">{"7"}</p>
          <p class="ant-scroll-number-only-unit">{"8"}</p>
          <p class="ant-scroll-number-only-unit">{"9"}</p>
        </>
      };
      let mut zero_class = String::from("");
      let mut one_class = String::from("");
      let mut two_class = String::from("");
      let mut three_class = String::from("");
      let mut four_class = String::from("");
      let mut five_class = String::from("");
      let mut six_class = String::from("");
      let mut seven_class = String::from("");
      let mut eight_class = String::from("");
      let mut nine_class = String::from("");
      let current = String::from("current");
      match count {
        0 => {zero_class = current;},
        1 => {one_class = current;},
        2 => {two_class = current;},
        3 => {three_class = current;},
        4 => {four_class = current;},
        5 => {five_class = current;},
        6 => {six_class = current;},
        7 => {seven_class = current;},
        8 => {eight_class = current;},
        9 => {nine_class = current;},
        _ => {
          zero_class = current.clone();
          one_class = current.clone();
          two_class = current.clone();
          three_class = current.clone();
          four_class = current.clone();
          five_class = current.clone();
          six_class = current.clone();
          seven_class = current.clone();
          eight_class = current.clone();
          nine_class = current;
        },//what sould i do in this case?
      };
      let p_class = String::from("ant-scroll-number-only-unit");
      let inner_html = match count_to_show.to_string().len() {
        1 => html!{
            <sup data-show="true" class="ant-scroll-number ant-badge-count" title={count.clone().to_string()}>
              <span class="ant-scroll-number-only" style={format!("transition: none 0s ease 0s; transform: translateY(-1{}00%);", count.to_string().chars().nth(0).unwrap().clone())}>
                {numbers.clone()}
                <p class={format!("{} {}", p_class, zero_class)}>{"0"}</p>
                <p class={format!("{} {}", p_class, one_class)}>{"1"}</p>
                <p class={format!("{} {}", p_class, two_class)}>{"2"}</p>
                <p class={format!("{} {}", p_class, three_class)}>{"3"}</p>
                <p class={format!("{} {}", p_class, four_class)}>{"4"}</p>
                <p class={format!("{} {}", p_class, five_class)}>{"5"}</p>
                <p class={format!("{} {}", p_class, six_class)}>{"6"}</p>
                <p class={format!("{} {}", p_class, seven_class)}>{"7"}</p>
                <p class={format!("{} {}", p_class, eight_class)}>{"8"}</p>
                <p class={format!("{} {}", p_class, nine_class)}>{"9"}</p>
                {numbers}
              </span>
            </sup>
        },
        2 => html!{
          <sup data-show="true" class="ant-scroll-number ant-badge-count ant-badge-multiple-words" title={count.clone().to_string()}>
            <span class="ant-scroll-number-only" style={format!("transition: none 0s ease 0s; transform: translateY(-1{}00%);", count.to_string().chars().nth(0).unwrap().clone())}>
              {numbers.clone()}
              <p class={format!("{} {}", p_class, zero_class)}>{"0"}</p>
              <p class={format!("{} {}", p_class, one_class)}>{"1"}</p>
              <p class={format!("{} {}", p_class, two_class)}>{"2"}</p>
              <p class={format!("{} {}", p_class, three_class)}>{"3"}</p>
              <p class={format!("{} {}", p_class, four_class)}>{"4"}</p>
              <p class={format!("{} {}", p_class, five_class)}>{"5"}</p>
              <p class={format!("{} {}", p_class, six_class)}>{"6"}</p>
              <p class={format!("{} {}", p_class, seven_class)}>{"7"}</p>
              <p class={format!("{} {}", p_class, eight_class)}>{"8"}</p>
              <p class={format!("{} {}", p_class, nine_class)}>{"9"}</p>
              {numbers.clone()}
            </span>
            <span class="ant-scroll-number-only" style={format!("transition: none 0s ease 0s; transform: translateY(-1{}00%);", count.to_string().chars().nth(1).unwrap().clone())}>
              {numbers.clone()}
              <p class={format!("{} {}", p_class, zero_class)}>{"0"}</p>
              <p class={format!("{} {}", p_class, one_class)}>{"1"}</p>
              <p class={format!("{} {}", p_class, two_class)}>{"2"}</p>
              <p class={format!("{} {}", p_class, three_class)}>{"3"}</p>
              <p class={format!("{} {}", p_class, four_class)}>{"4"}</p>
              <p class={format!("{} {}", p_class, five_class)}>{"5"}</p>
              <p class={format!("{} {}", p_class, six_class)}>{"6"}</p>
              <p class={format!("{} {}", p_class, seven_class)}>{"7"}</p>
              <p class={format!("{} {}", p_class, eight_class)}>{"8"}</p>
              <p class={format!("{} {}", p_class, nine_class)}>{"9"}</p>
              {numbers}
            </span>
          </sup>
        },
        _ => html!{
          <sup data-show="true" class="ant-scroll-number ant-badge-count" title={count.clone().to_string()}>
            <span class="ant-scroll-number-only" style={format!("transition: none 0s ease 0s; transform: translateY(-1{}00%);", count.to_string().chars().nth(0).unwrap().clone())}>
              {"99+"}
            </span>
          </sup>
        },
      };
      inner_html
    }
  };
  html! {
    <span class="ant-badge">
      <a href="#" class="head-example">
      </a>
      {sup}
    </span>
  }
}

