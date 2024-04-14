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

