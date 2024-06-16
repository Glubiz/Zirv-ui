//! Toast Component
//! 
//! This module provides a customizable `ToastComponent` for the Yew framework. The `ToastComponent` 
//! is used to display toast notifications with various properties and event handlers for click, 
//! mouse enter, and mouse leave events.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html, function_component, Html, Callback, MouseEvent};
//! use crate::components::toast::{ToastComponent, ToastComponentProps};
//! use crate::toast::Toast;
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     let toast = Toast {
//!         title: "Notification".to_string(),
//!         text: "This is a toast notification.".to_string(),
//!         ..Default::default()
//!     };
//! 
//!     let onclick = Callback::from(|_: MouseEvent| { /* handle click */ });
//!     let onenter = Callback::from(|_: MouseEvent| { /* handle mouse enter */ });
//!     let onleave = Callback::from(|_: MouseEvent| { /* handle mouse leave */ });
//! 
//!     html! {
//!         <ToastComponent toast={toast} {onclick} {onenter} {onleave} />
//!     }
//! }
//! ```

use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};
use crate::toast::utils::Notifiable;
use super::Toast;

/// Properties for the `ToastComponent`.
#[derive(Properties, Clone, PartialEq)]
pub struct ToastComponentProps {
    /// The `Toast` data to display.
    pub toast: Toast,
    /// Callback for click events.
    pub onclick: Callback<MouseEvent>,
    /// Callback for mouse enter events.
    pub onenter: Callback<MouseEvent>,
    /// Callback for mouse leave events.
    pub onleave: Callback<MouseEvent>,
}

/// The `ToastComponent`.
///
/// The `ToastComponent` is used to display a toast notification with customizable properties
/// and event handlers for click, mouse enter, and mouse leave events.
///
/// # Properties
///
/// - `toast`: The `Toast` data to display.
/// - `onclick`: Callback for click events.
/// - `onenter`: Callback for mouse enter events.
/// - `onleave`: Callback for mouse leave events.
#[function_component(ToastComponent)]
pub fn toast_component(props: &ToastComponentProps) -> Html {
    let title = &props.toast.title;
    let text = &props.toast.text;
    let toast_type = &props.toast.toast_type;

    let onclick = &props.onclick;
    let onenter = &props.onenter;
    let onleave = &props.onleave;

    let mut classes = vec![classes!("toast"), toast_type.into()];
    if props.toast.is_paused() {
        classes.push(classes!("paused"));
    }

    if let Some(additional_classes) = &props.toast.classes {
        classes.push(classes!(additional_classes));
    }

    html! {
        <div {onclick} onmouseenter={onenter} onmouseleave={onleave} class={classes}>
            <span class={classes!("toast-title")}>{title}</span>
            <span>{text}</span>
        </div>
    }
}
