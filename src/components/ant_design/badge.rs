use colorsys::Hsl;
use gloo::console::error;
use tufa_common::helpers::numeric::Numeric;
use yew::{function_component, html, Children, Html, Properties};

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

impl BadgeStatus {
    pub fn get_class(&self) -> String {
        match self {
            BadgeStatus::Success => String::from("ant-badge-status-success"),
            BadgeStatus::Processing => String::from("ant-badge-status-processing"),
            BadgeStatus::Default => String::from("ant-badge-status-default"),
            BadgeStatus::Error => String::from("ant-badge-status-error"),
            BadgeStatus::Warning => String::from("ant-badge-status-warning"),
        }
    }
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
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let offset_style = match props.offset.clone() {
        None => String::from(""),
        Some(badge_offset) => {
            let right_or_left = match badge_offset.x.is_positive() {
                true => format!("right: -{}px;", badge_offset.x),
                false => format!("right: {}px;", badge_offset.x.abs()),
            };
            let margin_top_or_bottom = match badge_offset.y.is_positive() {
                true => format!("margin-top: {}px;", badge_offset.y),
                false => format!("margin-top: {}px;", badge_offset.y),
            };
            format!("{} {}", right_or_left, margin_top_or_bottom)
        }
    };
    let status_dot_class = match props.status.clone() {
        None => String::from(""),
        Some(status) => status.get_class(),
    };
    let status_class = match props.status.clone() {
        None => String::from(""),
        Some(_) => String::from("ant-badge-status"),
    };
    let text_html = match (&props.text, props.status.clone()) {
        (None, None) => html! {},
        (None, Some(_)) => html! {},
        (Some(_), None) => html! {},
        (Some(text), Some(_)) => html! {<span class="ant-badge-status-text">{text}</span>},
    };
    let title = match (props.title.clone(), props.count) {
        (None, None) => String::from(""),
        (None, Some(count)) => count.to_string(),
        (Some(title), None) => title,
        (Some(title), Some(_)) => title,
    };
    let sup = match props.count {
        None => match (&props.color, &props.dot, &props.status) {
            (None, None, None) => html! {},
            (None, None, Some(_)) => {
                html! { <sup data-show="true" class={format!("ant-scroll-number ant-badge-dot {}", status_dot_class)} style={offset_style} title={title}></sup> }
            }
            (None, Some(_), None) => {
                html! { <sup data-show="true" class="ant-scroll-number ant-badge-dot" style={offset_style} title={title}></sup> }
            }
            (None, Some(_), Some(_)) => {
                html! { <sup data-show="true" class={format!("ant-scroll-number ant-badge-dot {}", status_dot_class)} style={offset_style} title={title}></sup> }
            }
            (Some(color), None, None) => {
                html! { <sup data-show="true" class="ant-scroll-number ant-badge-dot" style={format!("background: {}; {}", color.to_css_string(), offset_style)} title={title}></sup> }
            }
            (Some(color), None, Some(_)) => html! {
               <sup
                 data-show="true"
                 class={format!("ant-scroll-number ant-badge-dot {}", status_dot_class)}
                 style={format!("background: {}; {}", color.to_css_string(), offset_style)}
                 title={title}
               >
               </sup>
            },
            (Some(color), Some(_), None) => {
                html! { <sup data-show="true" class="ant-scroll-number ant-badge-dot" style={format!("background: {}; {}", color.to_css_string(), offset_style)} title={title}></sup> }
            }
            (Some(color), Some(_), Some(_)) => {
                html! {
                <sup
                  data-show="true"
                  class={format!("ant-scroll-number ant-badge-dot {}", status_dot_class)}
                  style={format!("background: {}; {}", color.to_css_string(), offset_style)}
                  title={title}
                >
                </sup> }
            }
        },
        Some(count) => {
            let max_count_number = props.overflow_count.unwrap_or(99);
            let count_to_show = count.to_string();
            let max_count_number_text = format!("{}+", max_count_number);
            let should_render = match (count == 0, props.show_zero) {
                (true, None) => false,
                (true, Some(_)) => false,
                (false, None) => true,
                (false, Some(_)) => true,
            };
            let numbers = count_to_show
                .chars()
                .map(|char| match Numeric::try_from(char) {
                    Err(char) => {
                        error!("badge component char is not a numeric: ", char.to_string());
                        html! {}
                    }
                    Ok(numeric) => html! {
                        <BadgeNumbers numeric={numeric}/>
                    },
                })
                .collect::<Vec<Html>>();
            let is_max_count_number_less = count > max_count_number;
            match (&props.color, &props.dot, &props.status) {
                (None, None, None) => {
                    if is_max_count_number_less {
                        html! {
                          <sup
                            data-show="true"
                            class="ant-scroll-number ant-badge-count ant-badge-multiple-words"
                            title={title}
                            style={offset_style}
                          >
                            {max_count_number_text}
                          </sup>
                        }
                    } else {
                        html! {
                          <>
                            if should_render {
                              <sup
                                data-show="true"
                                class="ant-scroll-number ant-badge-count"
                                title={title}
                                style={offset_style}
                              >
                                {for numbers}
                              </sup>
                            }
                          </>
                        }
                    }
                }
                (None, None, Some(_)) => {
                    html! { <sup data-show="true" class={format!("ant-scroll-number ant-badge-dot {}", status_dot_class)} style={offset_style} title={title}></sup> }
                }
                (None, Some(_), None) => {
                    html! { <sup data-show="true" class="ant-scroll-number ant-badge-dot" style={offset_style} title={title}></sup> }
                }
                (None, Some(_), Some(_)) => {
                    html! { <sup data-show="true" class={format!("ant-scroll-number ant-badge-dot {}", status_dot_class)} style={offset_style} title={title}></sup> }
                }
                (Some(color), None, None) => {
                    if count == 0 && props.show_zero.is_none() {
                        html! {}
                    } else if is_max_count_number_less {
                        html! {
                          <sup
                            data-show="true"
                            class="ant-scroll-number ant-badge-count ant-badge-multiple-words"
                            title={title}
                            style={format!("background: {}; {}", color.to_css_string(), offset_style)}
                          >
                            {max_count_number_text}
                          </sup>
                        }
                    } else {
                        html! {
                          <>
                            if should_render {
                              <sup
                                data-show="true"
                                class="ant-scroll-number ant-badge-count"
                                title={title}
                                style={format!("background: {}; {}", color.to_css_string(), offset_style)}
                              >
                                {for numbers}
                              </sup>
                            }
                          </>
                        }
                    }
                }
                (Some(color), None, Some(_)) => html! {
                   <sup
                     data-show="true"
                     class={format!("ant-scroll-number ant-badge-dot {}", status_dot_class)}
                     style={format!("background: {}; {}", color.to_css_string(), offset_style)}
                     title={title}
                   >
                   </sup>
                },
                (Some(color), Some(_), None) => html! {
                    <sup data-show="true" class="ant-scroll-number ant-badge-dot" style={format!("background: {}; {}", color.to_css_string(), offset_style)} title={title}></sup>
                },
                (Some(color), Some(_), Some(_)) => html! {
                    <sup data-show="true" class={format!("ant-scroll-number ant-badge-dot {}", status_dot_class)} style={format!("background: {}; {}", color.to_css_string(), offset_style)} title={title}></sup>
                },
            }
        }
    };
    html! {
      <span class={format!("ant-badge {}", status_class)}>
        { for props.children.iter() }
        {sup}
        {text_html}
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
