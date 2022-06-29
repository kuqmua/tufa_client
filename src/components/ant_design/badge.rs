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
        count_to_show = String::from("99+");
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
      let length = count_to_show.to_string().len();
      let inner_html = match length {
        1 => html!{
          <sup data-show="true" class="ant-scroll-number ant-badge-count" title={count.clone().to_string()}>
            <span class="ant-scroll-number-only" style="transition: none 0s ease 0s; transform: translateY(-1500%);">
              {numbers.clone()}
              <p class="ant-scroll-number-only-unit">{"0"}</p>
              <p class="ant-scroll-number-only-unit">{"1"}</p>
              <p class="ant-scroll-number-only-unit">{"2"}</p>
              <p class="ant-scroll-number-only-unit">{"3"}</p>
              <p class="ant-scroll-number-only-unit">{"4"}</p>
              <p class="ant-scroll-number-only-unit current">{"5"}</p>
              <p class="ant-scroll-number-only-unit">{"6"}</p>
              <p class="ant-scroll-number-only-unit">{"7"}</p>
              <p class="ant-scroll-number-only-unit">{"8"}</p>
              <p class="ant-scroll-number-only-unit">{"9"}</p>
              {numbers}
            </span>
          </sup>
        },
        2 => html!{
          <sup data-show="true" class="ant-scroll-number ant-badge-count ant-badge-multiple-words" title="98">
            <span class="ant-scroll-number-only" style="transition: none 0s ease 0s; transform: translateY(-1900%);">
              {numbers.clone()}
              <p class="ant-scroll-number-only-unit">{"1"}</p>
              <p class="ant-scroll-number-only-unit">{"2"}</p>
              <p class="ant-scroll-number-only-unit">{"3"}</p>
              <p class="ant-scroll-number-only-unit">{"4"}</p>
              <p class="ant-scroll-number-only-unit">{"5"}</p>
              <p class="ant-scroll-number-only-unit">{"6"}</p>
              <p class="ant-scroll-number-only-unit">{"7"}</p>
              <p class="ant-scroll-number-only-unit">{"8"}</p>
              <p class="ant-scroll-number-only-unit current">{"9"}</p>
              {numbers.clone()}
            </span>
            <span class="ant-scroll-number-only" style="transition: none 0s ease 0s; transform: translateY(-1800%);">
              {numbers.clone()}
              <p class="ant-scroll-number-only-unit">{"0"}</p>
              <p class="ant-scroll-number-only-unit">{"1"}</p>
              <p class="ant-scroll-number-only-unit">{"2"}</p>
              <p class="ant-scroll-number-only-unit">{"3"}</p>
              <p class="ant-scroll-number-only-unit">{"4"}</p>
              <p class="ant-scroll-number-only-unit">{"5"}</p>
              <p class="ant-scroll-number-only-unit">{"6"}</p>
              <p class="ant-scroll-number-only-unit">{"7"}</p>
              <p class="ant-scroll-number-only-unit current">{"8"}</p>
              <p class="ant-scroll-number-only-unit">{"9"}</p>
              {numbers}
            </span>
          </sup>
        },
        _ => html!{}//todo
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

