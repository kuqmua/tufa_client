use yewdux::prelude::{BasicStore, Dispatch};

#[derive(Clone, Default)]
pub struct YewduxStore {
    pub count: u32,
}

pub fn init() -> Dispatch<BasicStore<YewduxStore>> {
    Dispatch::<BasicStore<YewduxStore>>::new()
}
