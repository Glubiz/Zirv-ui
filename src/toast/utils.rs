//! Toast Utilities Module
//!
//! This module provides traits and utilities for managing and rendering toast notifications
//! in the Yew framework. It includes the `Notifiable` trait for defining toast notifications,
//! the `NotifiableComponentFactory` trait for creating toast components, and the `ToastType`
//! enum for categorizing toasts.
//!
//! # Example
//!
//! ```rust
//! use yew::{html, Callback, MouseEvent};
//! use crate::toast::utils::{Notifiable, NotifiableComponentFactory, ToastType};
//! use uuid::Uuid;
//! use time::Duration;
//!
//! #[derive(Clone, PartialEq)]
//! struct MyToast {
//!     id: Uuid,
//!     title: String,
//!     text: String,
//!     toast_type: ToastType,
//!     duration: i64,
//!     paused: bool,
//! }
//!
//! impl Notifiable for MyToast {
//!     fn id(&self) -> Uuid {
//!         self.id
//!     }
//!
//!     fn apply_tick(&mut self, duration: Duration) {
//!         if self.duration > 0 {
//!             self.duration -= duration.whole_milliseconds();
//!         }
//!     }
//!
//!     fn is_alive(&self) -> bool {
//!         self.duration > 0
//!     }
//!
//!     fn is_paused(&self) -> bool {
//!         self.paused
//!     }
//!
//!     fn mouse_in(&mut self) {
//!         self.paused = true;
//!     }
//!
//!     fn mouse_out(&mut self) {
//!         self.paused = false;
//!     }
//! }
//!
//! struct MyToastComponentFactory;
//!
//! impl NotifiableComponentFactory<MyToast> for MyToastComponentFactory {
//!     fn component(
//!         &self,
//!         toast: MyToast,
//!         onclick: Callback<MouseEvent>,
//!         onenter: Callback<MouseEvent>,
//!         onleave: Callback<MouseEvent>,
//!     ) -> Html {
//!         html! {
//!             <div class="toast" onclick={onclick} onmouseenter={onenter} onmouseleave={onleave}>
//!                 <strong>{ toast.title }</strong>
//!                 <p>{ toast.text }</p>
//!             </div>
//!         }
//!     }
//! }
//! ```

use std::any::Any;
use time::Duration;
use uuid::Uuid;
use yew::{classes, Callback, Classes, Html, MouseEvent};

/// Trait defining a notifiable toast notification.
///
/// This trait provides methods for managing the lifecycle of a toast notification,
/// including checking its ID, applying time ticks, checking if it is alive or paused,
/// and handling mouse events.
pub trait Notifiable: Any {
    /// Returns the unique ID of the toast notification.
    fn id(&self) -> Uuid;

    /// Applies a time tick to the toast, decreasing its remaining duration.
    ///
    /// # Parameters
    /// - `time`: The duration to decrease the toast's remaining time by.
    fn apply_tick(&mut self, time: Duration);

    /// Checks if the toast notification is still alive.
    ///
    /// # Returns
    /// - `true` if the toast is alive, `false` otherwise.
    fn is_alive(&self) -> bool;

    /// Checks if the toast notification is paused.
    ///
    /// # Returns
    /// - `true` if the toast is paused, `false` otherwise.
    fn is_paused(&self) -> bool;

    /// Pauses the toast notification (e.g., when the mouse enters).
    fn mouse_in(&mut self);

    /// Resumes the toast notification (e.g., when the mouse leaves).
    fn mouse_out(&mut self);
}

/// Trait defining a factory for creating notifiable toast components.
///
/// This trait provides a method for creating a toast component with the specified
/// callbacks for handling click, mouse enter, and mouse leave events.
pub trait NotifiableComponentFactory<T: Notifiable> {
    /// Creates a toast component with the specified callbacks.
    ///
    /// # Parameters
    /// - `toast`: The toast notification to create the component for.
    /// - `onclick`: Callback for handling click events.
    /// - `onenter`: Callback for handling mouse enter events.
    /// - `onleave`: Callback for handling mouse leave events.
    ///
    /// # Returns
    /// - An `Html` representation of the toast component.
    fn component(
        &self,
        toast: T,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html;
}

/// Enum representing the type of a toast notification.
///
/// The `ToastType` enum categorizes toast notifications into different types,
/// such as `Info`, `Warn`, and `Error`, which can be used to apply different
/// styles to the toasts.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ToastType {
    /// Informational toast notification.
    #[default]
    Info,

    /// Warning toast notification.
    Warn,

    /// Error toast notification.
    Error,
}

impl From<&str> for ToastType {
    /// Converts a string slice into a `ToastType`.
    ///
    /// # Parameters
    /// - `data`: The string slice to convert.
    ///
    /// # Returns
    /// - The corresponding `ToastType`, or `Info` if the string does not match any type.
    fn from(data: &str) -> Self {
        match data {
            "info" => Self::Info,
            "warn" => Self::Warn,
            "error" => Self::Error,
            _ => Self::Info,
        }
    }
}

impl From<&ToastType> for Classes {
    /// Converts a `ToastType` into a `Classes` for styling purposes.
    ///
    /// # Parameters
    /// - `toast_type`: The `ToastType` to convert.
    ///
    /// # Returns
    /// - The corresponding `Classes` for the `ToastType`.
    fn from(toast_type: &ToastType) -> Self {
        match toast_type {
            ToastType::Info => classes!("info"),
            ToastType::Warn => classes!("warn"),
            ToastType::Error => classes!("error"),
        }
    }
}
