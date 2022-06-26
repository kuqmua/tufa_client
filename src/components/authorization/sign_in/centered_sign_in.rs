use crate::components::authorization::center_form_wrapper::CenterFormWrapper;
use crate::components::authorization::sign_in::component::SignIn;
use yew::{function_component, html, ChildrenWithProps, Properties};

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct CenteredSignInProps {
    #[prop_or_default]
    children: ChildrenWithProps<SignIn>,
}

#[function_component(CenteredSignIn)]
pub fn centered_sign_in(props: &CenteredSignInProps) -> Html {
    html! {<CenterFormWrapper>{ for props.children.iter() }</CenterFormWrapper>}
}
