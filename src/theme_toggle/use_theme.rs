use yew::{hook, use_context};

use super::manager::ThemeManager;

#[hook]
pub fn use_theme<T: PartialEq + Clone>() -> ThemeManager<T> {
    use_context::<ThemeManager>().unwrap_or_default()
}