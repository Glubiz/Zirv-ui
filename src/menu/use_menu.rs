//! Menu State Hook Module
//!
//! This module provides a custom hook and associated types for managing menu state
//! in a Yew application. It leverages Yew's context system for state management.

use yew::{
    hook,
    use_context,
    Callback,
    MouseEvent,
};

/// Represents the state of a menu
#[derive(Clone, PartialEq)]
pub struct MenuState {
    /// Indicates whether the menu is currently open
    pub is_open: bool,
    /// Callback to toggle the menu's open/closed state
    pub toggle: Callback<MouseEvent>,
}

impl Default for MenuState {
    /// Provides a default state for the menu (closed with a no-op toggle)
    fn default() -> Self {
        Self { is_open: false, toggle: Callback::noop() }
    }
}

/// Custom hook to access the current menu state
///
/// This hook attempts to retrieve the `MenuState` from the current context.
/// If no context is found, it returns the default state.
///
/// # Returns
///
/// Returns the current `MenuState`, either from context or the default state.
///
/// # Example
///
/// ```
/// use your_crate::use_menu;
///
/// #[function_component(YourComponent)]
/// fn your_component() -> Html {
///     let menu_state = use_menu();
///
///     html! {
///         <div onclick={menu_state.toggle}>
///             { if menu_state.is_open { "Close Menu" } else { "Open Menu" } }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_menu() -> MenuState {
    use_context::<MenuState>().unwrap_or_default()
}
