use impl_display::ImplDisplayDerive;
use std::fmt;

#[derive(Debug, ImplDisplayDerive, Clone, PartialEq)]
pub enum RequestResult {
    NotExecuted,
    Pending,
    Success,
    Error,
}
