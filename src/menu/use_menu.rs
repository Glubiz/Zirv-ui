use yew::{hook, use_context};

use super::manager::MenuManager;

#[hook]
pub fn use_menu<T: PartialEq + Clone>() -> MenuManager<T> {
    use_context::<MenuManager<T>>().unwrap_or_default()
}
