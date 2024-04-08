use std::fmt::Debug;
use std::rc::Rc;

use time::Duration;
use uuid::Uuid;
use yew::{Reducible, UseReducerDispatcher};

use super::utils::Notifiable;

#[derive(Clone, PartialEq)]
pub struct ToastManager<T: Notifiable + PartialEq + Clone> {
    pub(crate) sender: Option<UseReducerDispatcher<ToastsList<T>>>,
}

impl<T: Notifiable + PartialEq + Clone> ToastManager<T> {
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

#[derive(Debug)]
pub enum Action<T: Notifiable + PartialEq + Clone> {
    New(T),
    Close(Uuid),
    Tick,
    Pause(Uuid),
    Continue(Uuid),
}

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

    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }
}

impl<T: Notifiable + PartialEq + Clone> Reducible for ToastsList<T> {
    type Action = Action<T>;

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
