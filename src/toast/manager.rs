//! Toast Management Module
//!
//! This module provides functionality for managing toast notifications in a Yew application. 
//! It includes the `ToastManager` to manage the creation and dispatching of toast notifications 
//! and the `ToastsList` to handle the collection and state updates of toasts.
//! 
//! # Example
//!
//! ```rust
//! use yew::{html, Callback, MouseEvent};
//! use crate::components::toast::{ToastManager, ToastsList, Action, Toast};
//! use uuid::Uuid;
//!
//! #[derive(Clone, PartialEq)]
//! struct MyToast {
//!     id: Uuid,
//!     title: String,
//!     text: String,
//!     is_paused: bool,
//!     duration: i64,
//! }
//!
//! impl Toast for MyToast {
//!     fn id(&self) -> Uuid {
//!         self.id
//!     }
//!
//!     fn text(&self) -> &str {
//!         &self.text
//!     }
//!
//!     fn is_paused(&self) -> bool {
//!         self.is_paused
//!     }
//!
//!     fn apply_tick(&mut self, duration: time::Duration) {
//!         if self.duration > 0 {
//!             self.duration -= duration.whole_milliseconds();
//!         }
//!     }
//!
//!     fn is_alive(&self) -> bool {
//!         self.duration > 0
//!     }
//!
//!     fn mouse_in(&mut self) {
//!         self.is_paused = true;
//!     }
//!
//!     fn mouse_out(&mut self) {
//!         self.is_paused = false;
//!     }
//! }
//!
//! let toast_manager = ToastManager::<MyToast>::default();
//! let toast = MyToast {
//!     id: Uuid::new_v4(),
//!     title: "New Toast".to_string(),
//!     text: "This is a toast notification".to_string(),
//!     is_paused: false,
//!     duration: 5000,
//! };
//!
//! toast_manager.spawn(toast);
//! ```

use std::fmt::Debug;
use std::rc::Rc;
use time::Duration;
use uuid::Uuid;
use yew::{Reducible, UseReducerDispatcher};
use super::utils::Notifiable;

/// The `ToastManager` is responsible for managing the creation and dispatching of toast notifications.
///
/// # Properties
/// - `sender`: An optional dispatcher to send actions to the `ToastsList`.
#[derive(Clone, PartialEq)]
pub struct ToastManager<T: Notifiable + PartialEq + Clone> {
    pub(crate) sender: Option<UseReducerDispatcher<ToastsList<T>>>,
}

impl<T: Notifiable + PartialEq + Clone> ToastManager<T> {
    /// Spawns a new toast notification.
    ///
    /// # Parameters
    /// - `toast`: The toast notification to be added.
    pub fn spawn(&self, toast: T) {
        if let Some(sender) = &self.sender {
            sender.dispatch(Action::New(toast));
        }
    }
}

impl<T: Notifiable + PartialEq + Clone> Default for ToastManager<T> {
    fn default() -> Self {
        Self {
            sender: Default::default(),
        }
    }
}

/// Actions that can be performed on the `ToastsList`.
///
/// # Parameters
/// - `New(toast)`: Adds a new toast notification.
/// - `Close(id)`: Closes a toast notification by its ID.
/// - `Tick`: Updates the state of all toasts based on the elapsed time.
/// - `Pause(id)`: Pauses a toast notification by its ID.
/// - `Continue(id)`: Continues a toast notification by its ID.
#[derive(Debug)]
pub enum Action<T: Notifiable + PartialEq + Clone> {
    New(T),
    Close(Uuid),
    Tick,
    Pause(Uuid),
    Continue(Uuid),
}

/// A list of toast notifications.
///
/// # Properties
/// - `toasts`: A vector containing all the toast notifications.
#[derive(Debug, Clone, PartialEq)]
pub struct ToastsList<T> {
    pub toasts: Vec<T>,
}

impl<T> Default for ToastsList<T> {
    fn default() -> Self {
        Self {
            toasts: Default::default(),
        }
    }
}

impl<T> ToastsList<T> {
    pub const TIME_TICK_MILLIS: usize = 100;
    pub const TIME_TICK_DURATION: Duration = Duration::milliseconds(100);

    /// Checks if the list of toasts is empty.
    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }
}

impl<T: Notifiable + PartialEq + Clone> Reducible for ToastsList<T> {
    type Action = Action<T>;

    /// Reduces the state based on the action received.
    ///
    /// # Parameters
    /// - `action`: The action to perform on the toast list.
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::New(toast) => {
                let mut toasts = self.toasts.clone();
                toasts.push(toast);

                Rc::new(Self { toasts })
            }
            Action::Close(id) => {
                let toasts = self
                    .toasts
                    .clone()
                    .into_iter()
                    .filter(|t| t.id() != id)
                    .collect();

                Rc::new(Self { toasts })
            }
            Action::Tick => {
                let toasts = self
                    .toasts
                    .clone()
                    .into_iter()
                    .filter_map(|mut t| {
                        if t.is_paused() {
                            Some(t)
                        } else if t.is_alive() {
                            t.apply_tick(Self::TIME_TICK_DURATION);

                            Some(t)
                        } else {
                            None
                        }
                    })
                    .collect();

                Rc::new(Self { toasts })
            }
            Action::Pause(id) => {
                let toasts = self
                    .toasts
                    .clone()
                    .into_iter()
                    .map(|mut t| {
                        if t.id() == id {
                            t.mouse_in();
                        }
                        t
                    })
                    .collect();

                Rc::new(Self { toasts })
            }
            Action::Continue(id) => {
                let toasts = self
                    .toasts
                    .clone()
                    .into_iter()
                    .map(|mut t| {
                        if t.id() == id {
                            t.mouse_out();
                        }
                        t
                    })
                    .collect();

                Rc::new(Self { toasts })
            }
        }
    }
}
