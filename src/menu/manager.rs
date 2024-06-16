use std::fmt::Debug;
use std::rc::Rc;

use uuid::Uuid;
use yew::{Reducible, UseReducerDispatcher};

use super::Menu;

#[derive(Clone, PartialEq)]
pub struct MenuManager<T: PartialEq + Clone> {
    pub(crate) sender: Option<UseReducerDispatcher<Menu>>,
}

impl<T: PartialEq + Clone> MenuManager<T> {
    pub fn spawn(&self, toast: T) {
        if let Some(sender) = &self.sender {
            sender.dispatch(Action::New(toast));
        }
    }
}

impl<T: PartialEq + Clone> Default for MenuManager<T> {
    fn default() -> Self {
        Self {
            sender: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Action<T: PartialEq + Clone> {
    New(T),
    Close(Uuid),
}

impl<T: PartialEq + Clone> Reducible for Menu {
    type Action = Action<T>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::New(toast) => {
                let mut toasts = self.toasts.clone();
                toasts.push(toast);

                Rc::new(Self { toasts })
            }
            Action::Close(id) => {
                let toasts = self.toasts.clone().into_iter().filter(|t| t.id() != id).collect();

                Rc::new(Self { toasts })
            }
        }
    }
}
