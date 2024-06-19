//! Toast Hook
//!
//! This module provides a custom hook `use_toast` for the Yew framework. The `use_toast` hook allows
//! components to easily access the `ToastManager` context for managing toast notifications.

use super::manager::ToastManager;
use super::utils::Notifiable;
use yew::{hook, use_context};

/// Custom hook to access the `ToastManager` context.
///
/// The `use_toast` hook allows components to easily access the `ToastManager` context for managing toast notifications.
///
/// # Returns
///
/// Returns a `ToastManager` instance.
///
#[hook]
pub fn use_toast<T>() -> ToastManager<T>
where
    T: Notifiable + PartialEq + Clone,
{
    use_context::<ToastManager<T>>().unwrap_or_default()
}
