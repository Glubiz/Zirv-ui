use std::fmt::Debug;
use std::rc::Rc;

use super::provider::Theme;

#[derive(Clone, PartialEq)]
pub struct ThemeManager {
    pub(crate) theme: Theme
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            theme: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Toggle,
}

impl Reducible for ThemeManager {
    type Action = Action;

    fn reduce(&mut self, action: Self::Action) {
        match action {
            Action::Toggle => {
                *self = match self {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                }
            }
        }
    }
}