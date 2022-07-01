use colorsys::Hsl;
use yew::{function_component, html, Properties};

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
    pub color: Option<Hsl>,          // Customize Badge dot color	string	-	3.16.0
    pub count: Option<i64>,          //	Number to show in badge	ReactNode
    pub dot: Option<bool>,           // Whether to display a red dot instead of count	boolean	false
    pub offset: Option<BadgeOffset>, //	set offset of the badge dot, like[x, y]	[number, number]	-
    // pub overflow_count: Option<i64>,//dont think it would be usefull//	Max count to show	number	99
    pub show_zero: Option<bool>, //	Whether to show badge when count is zero	boolean	false
    pub status: Option<BadgeStatus>, //	Set Badge as a status dot	success | processing | default | error | warning	''
    pub text: Option<String>, //	If status is set, text sets the display text of the status dot	string	''
    pub title: Option<String>, //	Text to show when hovering over the badge	string	count
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let sup = match props.count {
        None => html! {},
        Some(count) => {
            let count_to_show = count.to_string();
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
            let empty_string = String::from("");
            let current = String::from("current");
            let p_class = String::from("ant-scroll-number-only-unit");
            let inner_html = match count_to_show.len() {
                1 => {
                    let first_element = count_to_show.chars().nth(0).unwrap();
                    let mut zero_class = empty_string.clone();
                    let mut one_class = empty_string.clone();
                    let mut two_class = empty_string.clone();
                    let mut three_class = empty_string.clone();
                    let mut four_class = empty_string.clone();
                    let mut five_class = empty_string.clone();
                    let mut six_class = empty_string.clone();
                    let mut seven_class = empty_string.clone();
                    let mut eight_class = empty_string.clone();
                    let mut nine_class = empty_string.clone();
                    match first_element {
                        '0' => {
                            zero_class = current;
                        }
                        '1' => {
                            one_class = current;
                        }
                        '2' => {
                            two_class = current;
                        }
                        '3' => {
                            three_class = current;
                        }
                        '4' => {
                            four_class = current;
                        }
                        '5' => {
                            five_class = current;
                        }
                        '6' => {
                            six_class = current;
                        }
                        '7' => {
                            seven_class = current;
                        }
                        '8' => {
                            eight_class = current;
                        }
                        '9' => {
                            nine_class = current;
                        }
                        _ => (),
                    };
                    let numbers_to_scroll = html! {
                      <>
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
                      </>
                    };
                    html! {
                      <sup data-show="true" class="ant-scroll-number ant-badge-count" title={count_to_show.clone()}>
                        <span class="ant-scroll-number-only" style={format!("transition: none 0s ease 0s; transform: translateY(-1{}00%);", first_element.clone())}>
                          {numbers_to_scroll}
                        </span>
                      </sup>
                    }
                }
                2 => {
                    let first_element = count_to_show.chars().nth(0).unwrap();
                    let mut first_zero_class = empty_string.clone();
                    let mut first_one_class = empty_string.clone();
                    let mut first_two_class = empty_string.clone();
                    let mut first_three_class = empty_string.clone();
                    let mut first_four_class = empty_string.clone();
                    let mut first_five_class = empty_string.clone();
                    let mut first_six_class = empty_string.clone();
                    let mut first_seven_class = empty_string.clone();
                    let mut first_eight_class = empty_string.clone();
                    let mut first_nine_class = empty_string.clone();
                    match first_element {
                        '0' => {
                            first_zero_class = current.clone();
                        }
                        '1' => {
                            first_one_class = current.clone();
                        }
                        '2' => {
                            first_two_class = current.clone();
                        }
                        '3' => {
                            first_three_class = current.clone();
                        }
                        '4' => {
                            first_four_class = current.clone();
                        }
                        '5' => {
                            first_five_class = current.clone();
                        }
                        '6' => {
                            first_six_class = current.clone();
                        }
                        '7' => {
                            first_seven_class = current.clone();
                        }
                        '8' => {
                            first_eight_class = current.clone();
                        }
                        '9' => {
                            first_nine_class = current.clone();
                        }
                        _ => (),
                    };
                    let first_number_to_scroll = html! {
                      <>
                        {numbers.clone()}
                        <p class={format!("{} {}", p_class, first_zero_class)}>{"0"}</p>
                        <p class={format!("{} {}", p_class, first_one_class)}>{"1"}</p>
                        <p class={format!("{} {}", p_class, first_two_class)}>{"2"}</p>
                        <p class={format!("{} {}", p_class, first_three_class)}>{"3"}</p>
                        <p class={format!("{} {}", p_class, first_four_class)}>{"4"}</p>
                        <p class={format!("{} {}", p_class, first_five_class)}>{"5"}</p>
                        <p class={format!("{} {}", p_class, first_six_class)}>{"6"}</p>
                        <p class={format!("{} {}", p_class, first_seven_class)}>{"7"}</p>
                        <p class={format!("{} {}", p_class, first_eight_class)}>{"8"}</p>
                        <p class={format!("{} {}", p_class, first_nine_class)}>{"9"}</p>
                        {numbers.clone()}
                      </>
                    };
                    let second_element = count_to_show.chars().nth(1).unwrap();
                    let mut second_zero_class = empty_string.clone();
                    let mut second_one_class = empty_string.clone();
                    let mut second_two_class = empty_string.clone();
                    let mut second_three_class = empty_string.clone();
                    let mut second_four_class = empty_string.clone();
                    let mut second_five_class = empty_string.clone();
                    let mut second_six_class = empty_string.clone();
                    let mut second_seven_class = empty_string.clone();
                    let mut second_eight_class = empty_string.clone();
                    let mut second_nine_class = empty_string.clone();
                    match second_element {
                        '0' => {
                            second_zero_class = current;
                        }
                        '1' => {
                            second_one_class = current;
                        }
                        '2' => {
                            second_two_class = current;
                        }
                        '3' => {
                            second_three_class = current;
                        }
                        '4' => {
                            second_four_class = current;
                        }
                        '5' => {
                            second_five_class = current;
                        }
                        '6' => {
                            second_six_class = current;
                        }
                        '7' => {
                            second_seven_class = current;
                        }
                        '8' => {
                            second_eight_class = current;
                        }
                        '9' => {
                            second_nine_class = current;
                        }
                        _ => (),
                    };
                    let second_number_to_scroll = html! {
                      <>
                        {numbers.clone()}
                        <p class={format!("{} {}", p_class, second_zero_class)}>{"0"}</p>
                        <p class={format!("{} {}", p_class, second_one_class)}>{"1"}</p>
                        <p class={format!("{} {}", p_class, second_two_class)}>{"2"}</p>
                        <p class={format!("{} {}", p_class, second_three_class)}>{"3"}</p>
                        <p class={format!("{} {}", p_class, second_four_class)}>{"4"}</p>
                        <p class={format!("{} {}", p_class, second_five_class)}>{"5"}</p>
                        <p class={format!("{} {}", p_class, second_six_class)}>{"6"}</p>
                        <p class={format!("{} {}", p_class, second_seven_class)}>{"7"}</p>
                        <p class={format!("{} {}", p_class, second_eight_class)}>{"8"}</p>
                        <p class={format!("{} {}", p_class, second_nine_class)}>{"9"}</p>
                        {numbers}
                      </>
                    };
                    html! {
                      <sup data-show="true" class="ant-scroll-number ant-badge-count ant-badge-multiple-words" title={count_to_show.clone()}>
                        <span class="ant-scroll-number-only" style={format!("transition: none 0s ease 0s; transform: translateY(-1{}00%);", first_element)}>
                          {first_number_to_scroll}
                        </span>
                        <span class="ant-scroll-number-only" style={format!("transition: none 0s ease 0s; transform: translateY(-1{}00%);", second_element)}>
                          {second_number_to_scroll}
                        </span>
                      </sup>
                    }
                }
                _ => html! {
                  <sup data-show="true" class="ant-scroll-number ant-badge-count ant-badge-multiple-words" title={count_to_show.clone()}>{"99+"}</sup>
                },
            };
            inner_html
        }
    };
    html! {
      <span class="ant-badge">
        <a href="" class="head-example">
        </a>
        {sup}
      </span>
    }
}
