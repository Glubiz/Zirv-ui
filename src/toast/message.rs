//! Toast Component
//!
//! This module provides a simple toast notification component for the Yew framework. The `Toast`
//! component displays a message with a specific type that determines its styling.

use yew::prelude::*;

/// Properties for the `Toast` component.
#[derive(Properties, PartialEq)]
pub struct ToastProps {
    /// The message to be displayed in the toast.
    pub message: String,
    /// The type of the toast, which determines its styling.
    pub toast_type: String,
}

/// The `Toast` component.
///
/// The `Toast` component displays a message with a specific type that determines its styling. The
/// type can be used to apply different CSS classes for styling purposes.
///
/// # Properties
///
/// - `message`: The message to be displayed in the toast.
/// - `toast_type`: The type of the toast, which determines its styling.
#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    html! {
        <div class={format!("toast {}", props.toast_type)}>
            { &props.message }
        </div>
    }
}
