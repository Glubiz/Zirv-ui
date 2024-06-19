//! Toast Hook
//! 
//! This module provides a custom hook `use_toast` for the Yew framework. The `use_toast` hook allows
//! components to easily access the `ToastManager` context for managing toast notifications.

use yew::{hook, use_context};
use super::manager::ToastManager;
use super::utils::Notifiable; 

/// Custom hook to access the `ToastManager` context.
///
/// The `use_toast` hook allows components to easily access the `ToastManager` context for managing toast notifications.
///
/// # Returns
///
/// Returns a `ToastManager` instance.
///
/// # Example
///
/// ```rust
/// use yew::{html, function_component, Html};
/// use crate::hooks::use_toast;
/// use crate::toast::{Toast, ToastType};
/// use uuid::Uuid;
///
/// #[derive(Clone, PartialEq)]
/// struct MyToast {
///     id: Uuid,
///     title: String,
///     text: String,
///     is_paused: bool,
///     duration: i64,
/// }
///
/// impl Toast for MyToast {
///     fn id(&self) -> Uuid {
///         self.id
///     }
///
///     fn text(&self) -> &str {
///         &self.text
///     }
///
///     fn is_paused(&self) -> bool {
///         self.is_paused
///     }
///
///     fn apply_tick(&mut self, duration: time::Duration) {
///         if self.duration > 0 {
///             self.duration -= duration.whole_milliseconds();
///         }
///     }
///
///     fn is_alive(&self) -> bool {
///         self.duration > 0
///     }
///
///     fn mouse_in(&mut self) {
///         self.is_paused = true;
///     }
///
///     fn mouse_out(&mut self) {
///         self.is_paused = false;
///     }
/// }
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let toast_manager = use_toast::<MyToast>();
///
///     let show_toast = {
///         let toast_manager = toast_manager.clone();
///         move || {
///             let toast = MyToast {
///                 id: Uuid::new_v4(),
///                 title: "Notification".to_string(),
///                 text: "This is a toast notification.".to_string(),
///                 is_paused: false,
///                 duration: 5000,
///             };
///             toast_manager.spawn(toast);
///         }
///     };
///
///     html! {
///         <button onclick={show_toast}>{"Show Toast"}</button>
///     }
/// }
/// ```
#[hook]
pub fn use_toast<T: Notifiable + PartialEq + Clone>() -> ToastManager<T> {
    use_context::<ToastManager<T>>().unwrap_or_default()
}
