use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    SignUp,
    #[at("/sign_in")]
    SignIn,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/counter")]
    CounterHandle,
    #[at("/example")]
    Example
}
