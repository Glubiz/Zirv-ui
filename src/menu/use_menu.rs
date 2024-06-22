use yew::{
    hook,
    use_context,
    Callback,
    MouseEvent,
};

#[derive(Clone, PartialEq)]
pub struct MenuState {
    pub is_open: bool,
    pub toggle: Callback<MouseEvent>,
}

impl Default for MenuState {
    fn default() -> Self {
        Self { is_open: false, toggle: Callback::noop() }
    }
}

#[hook]
pub fn use_menu() -> MenuState {
    use_context::<MenuState>().unwrap_or_default()
}
