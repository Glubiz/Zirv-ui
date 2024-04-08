use yew::{hook, use_context};

use super::manager::ToastManager;
use super::utils::Notifiable; 

#[hook]
pub fn use_toast<T: Notifiable + PartialEq + Clone>() -> ToastManager<T> {
    use_context::<ToastManager<T>>().unwrap_or_default()
}