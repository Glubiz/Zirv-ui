//! Toast Factory
//!
//! This module defines the `ToastFactory` struct and implements the `NotifiableComponentFactory`
//! trait for creating `Toast` components in the Yew framework. The `ToastFactory` is responsible
//! for generating `ToastComponent` instances with specific callbacks for click, mouse enter, and
//! mouse leave events.

use yew::{
    html,
    Callback,
    Html,
    MouseEvent,
};

use super::{
    component::ToastComponent,
    utils::NotifiableComponentFactory,
    Toast,
};

/// A factory for creating `ToastComponent` instances.
#[derive(Clone, PartialEq, Default)]
pub struct ToastFactory;

impl NotifiableComponentFactory<Toast> for ToastFactory {
    /// Creates a `ToastComponent` with the given `Toast` data and event callbacks.
    ///
    /// # Parameters
    ///
    /// - `toast`: The `Toast` data to display.
    /// - `onclick`: Callback for click events.
    /// - `onenter`: Callback for mouse enter events.
    /// - `onleave`: Callback for mouse leave events.
    ///
    /// # Returns
    ///
    /// Returns a `Html` containing the `ToastComponent`.
    fn component(
        &self,
        toast: Toast,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <ToastComponent {toast} {onclick} {onenter} {onleave} />
        }
    }
}
