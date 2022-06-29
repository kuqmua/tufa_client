use yew::{function_component, html, Properties};

#[derive(Debug, PartialEq, Clone)]
pub struct BadgeOffset {
    pub x: i32,
    pub y: i32,
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
    pub color: Option<String>,// Customize Badge dot color	string	-	3.16.0
    pub count: Option<i32>,//	Number to show in badge	ReactNode		
    pub dot: Option<bool>,// Whether to display a red dot instead of count	boolean	false	
    pub offset: Option<BadgeOffset>,//	set offset of the badge dot, like[x, y]	[number, number]	-	
    pub overflow_count: Option<i32>,//	Max count to show	number	99	
    pub show_zero: Option<bool>,//	Whether to show badge when count is zero	boolean	false	
    pub status: Option<BadgeStatus>,//	Set Badge as a status dot	success | processing | default | error | warning	''	
    pub text: Option<String>,//	If status is set, text sets the display text of the status dot	string	''	
    pub title: Option<String>,//	Text to show when hovering over the badge	string	count
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    html! {
        <span class="ant-badge">
          <a href="#" class="head-example">
          </a>
          <sup data-show="true" class="ant-scroll-number ant-badge-count" title="5">
            <span class="ant-scroll-number-only" style="transition: none 0s ease 0s; transform: translateY(-1500%);">
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
            </span>
          </sup>
        </span>
    }
}

