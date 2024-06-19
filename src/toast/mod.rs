//! Toast Module
//!
//! This module provides the `Toast` struct, which represents a toast notification, and implements the `Notifiable`
//! trait for managing its lifecycle. Toast notifications are used to display brief messages to the user and can be
//! customized in terms of type, title, text, and appearance. The module also includes various submodules for

use self::utils::{Notifiable, ToastType};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

pub mod component;
pub mod component_factory;
pub mod manager;
pub mod message;
pub mod provider;
pub mod use_toast;
pub mod utils;

/// Represents a toast notification.
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    pub(crate) id: Uuid,
    pub(crate) toast_type: ToastType,
    pub(crate) title: String,
    pub(crate) text: String,
    pub(crate) classes: Option<String>,
    pub(crate) spawn_time: OffsetDateTime,
    pub(crate) lifetime: Duration,
    pub(crate) full_lifetime: Duration,
    pub(crate) paused: bool,
}

impl Toast {
    /// Default lifetime of a toast notification.
    pub const LIFETIME: Duration = Duration::milliseconds(3000);

    /// Creates a new `Toast` notification.
    ///
    /// # Parameters
    ///
    /// - `toast_type`: The type of the toast (e.g., success, error, info).
    /// - `title`: The title of the toast notification.
    /// - `text`: The text content of the toast notification.
    ///
    /// # Returns
    ///
    /// Returns a new `Toast` instance.
    pub fn new(toast_type: ToastType, title: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            toast_type,
            title: title.into(),
            text: text.into(),
            classes: Some("toast-loading".to_owned()),
            spawn_time: OffsetDateTime::now_local().expect("Unable to get current time"),
            lifetime: Self::LIFETIME,
            full_lifetime: Self::LIFETIME,
            paused: false,
        }
    }
}

impl Notifiable for Toast {
    /// Returns the ID of the toast.
    fn id(&self) -> Uuid {
        self.id
    }

    /// Applies a tick to the toast, decreasing its lifetime.
    ///
    /// # Parameters
    ///
    /// - `time`: The duration to decrease the lifetime by.
    fn apply_tick(&mut self, time: Duration) {
        self.lifetime = self.lifetime.checked_sub(time).unwrap_or_default();

        if self.lifetime < Self::LIFETIME - Duration::milliseconds(500) {
            self.classes = None;
        }
    }

    /// Checks if the toast is still alive (i.e., its lifetime is not zero).
    ///
    /// # Returns
    ///
    /// Returns `true` if the toast is alive, `false` otherwise.
    fn is_alive(&self) -> bool {
        self.lifetime != Duration::default()
    }

    /// Pauses the toast (e.g., when the mouse hovers over it).
    fn mouse_in(&mut self) {
        self.paused = true;
    }

    /// Resumes the toast (e.g., when the mouse leaves it).
    fn mouse_out(&mut self) {
        self.paused = false;
        self.lifetime = self.full_lifetime;
    }

    /// Checks if the toast is paused.
    ///
    /// # Returns
    ///
    /// Returns `true` if the toast is paused, `false` otherwise.
    fn is_paused(&self) -> bool {
        self.paused
    }
}
