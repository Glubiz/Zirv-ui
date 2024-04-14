use web_sys::{window, HtmlElement};
use yew::{prelude::*, ContextProvider, Properties, html, Callback, Children};

#[derive(Clone, PartialEq, Debug)]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: Theme,
}


