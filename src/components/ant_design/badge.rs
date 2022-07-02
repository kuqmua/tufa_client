use colorsys::Hsl;
use gloo::console::error;
use tufa_common::helpers::numeric::Numeric;
use yew::{function_component, html, Html, Properties};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BadgeOffset {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BadgeStatus {
    Success,
    Processing,
    Default,
    Error,
    Warning,
}

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub color: Option<Hsl>,          // Customize Badge dot color	string	-	3.16.0
    pub count: Option<u64>,          //	Number to show in badge	ReactNode
    pub dot: Option<()>,             // Whether to display a red dot instead of count	boolean	false
    pub offset: Option<BadgeOffset>, //	set offset of the badge dot, like[x, y]	[number, number]	-
    pub overflow_count: Option<u64>, //dont think it would be usefull//	Max count to show	number	99
    pub show_zero: Option<()>,       //	Whether to show badge when count is zero	boolean	false
    pub status: Option<BadgeStatus>, //	Set Badge as a status dot	success | processing | default | error | warning	''
    pub text: Option<String>, //	If status is set, text sets the display text of the status dot	string	''
    pub title: Option<String>, //	Text to show when hovering over the badge	string	count
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let sup = match props.count {
        None => match (&props.color, &props.dot) {
            (None, None) => html! {},
            (None, Some(_)) => html! {
              <sup data-show="true" class="ant-scroll-number ant-badge-dot"></sup>
            },
            (Some(color), None) => html! {
              <sup data-show="true" class="ant-scroll-number ant-badge-dot" style={format!("background: {};", color.to_css_string())}></sup>
            },
            (Some(color), Some(_)) => html! {
              <sup data-show="true" class="ant-scroll-number ant-badge-dot" style={format!("background: {};", color.to_css_string())}></sup>
            },
        },
        Some(count) => match (&props.color, &props.dot) {
            (None, None) => {
                let max_count_number = props.overflow_count.unwrap_or(99);
                let count_to_show = count.to_string();
                match count > max_count_number {
                    true => html! {
                      <sup data-show="true" class="ant-scroll-number ant-badge-count ant-badge-multiple-words" title={count_to_show.clone()}>{format!("{}+", max_count_number)}</sup>
                    },
                    false => {
                        let numbers = count_to_show
                            .chars()
                            .map(|char| match Numeric::try_from(char) {
                                Err(char) => {
                                    error!(
                                        "badge component char is not a numeric: ",
                                        char.to_string()
                                    );
                                    html! {}
                                }
                                Ok(numeric) => html! {
                                    <BadgeNumbers numeric={numeric}/>
                                },
                            })
                            .collect::<Vec<Html>>();
                        html! {
                          <sup data-show="true" class="ant-scroll-number ant-badge-count" title={count_to_show.clone()}>
                            {for numbers}
                          </sup>
                        }
                    }
                }
            }
            (None, Some(_)) => html! {
              <sup data-show="true" class="ant-scroll-number ant-badge-dot"></sup>
            },
            (Some(color), None) => html! {
              <sup data-show="true" class="ant-scroll-number ant-badge-dot" style={format!("background: {};", color.to_css_string())}></sup>
            },
            (Some(color), Some(_)) => html! {
              <sup data-show="true" class="ant-scroll-number ant-badge-dot" style={format!("background: {};", color.to_css_string())}></sup>
            },
        },
    };
    html! {
      <span class="ant-badge">
        <a href="" class="head-example">
        </a>
        {sup}
      </span>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct BadgeNumbersProps {
    pub numeric: Numeric,
}

#[function_component(BadgeNumbers)]
pub fn badge_numbers(props: &BadgeNumbersProps) -> Html {
    let numbers = html! {
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
    let current = String::from("current");
    let p_class = String::from("ant-scroll-number-only-unit");
    let mut zero_class = p_class.clone();
    let mut one_class = p_class.clone();
    let mut two_class = p_class.clone();
    let mut three_class = p_class.clone();
    let mut four_class = p_class.clone();
    let mut five_class = p_class.clone();
    let mut six_class = p_class.clone();
    let mut seven_class = p_class.clone();
    let mut eight_class = p_class.clone();
    let mut nine_class = p_class.clone();
    match props.numeric.clone() {
        Numeric::Zero => {
            zero_class = format!("{} {}", p_class, current);
        }
        Numeric::One => {
            one_class = format!("{} {}", p_class, current);
        }
        Numeric::Two => {
            two_class = format!("{} {}", p_class, current);
        }
        Numeric::Three => {
            three_class = format!("{} {}", p_class, current);
        }
        Numeric::Four => {
            four_class = format!("{} {}", p_class, current);
        }
        Numeric::Five => {
            five_class = format!("{} {}", p_class, current);
        }
        Numeric::Six => {
            six_class = format!("{} {}", p_class, current);
        }
        Numeric::Seven => {
            seven_class = format!("{} {}", p_class, current);
        }
        Numeric::Eight => {
            eight_class = format!("{} {}", p_class, current);
        }
        Numeric::Nine => {
            nine_class = format!("{} {}", p_class, current);
        }
    };
    html! {
       <span class="ant-scroll-number-only" style={format!("transition: none 0s ease 0s; transform: translateY(-1{}00%);", props.numeric.clone())}>
         {numbers.clone()}
         <p class={zero_class}>{"0"}</p>
         <p class={one_class}>{"1"}</p>
         <p class={two_class}>{"2"}</p>
         <p class={three_class}>{"3"}</p>
         <p class={four_class}>{"4"}</p>
         <p class={five_class}>{"5"}</p>
         <p class={six_class}>{"6"}</p>
         <p class={seven_class}>{"7"}</p>
         <p class={eight_class}>{"8"}</p>
         <p class={nine_class}>{"9"}</p>
         {numbers.clone()}
       </span>
    }
}
