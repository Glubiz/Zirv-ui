use crate::pages::{about::About, index::Index};

use yew::{
    function_component, html, html_nested, use_callback, use_effect_with, use_state_eq, Children,
    Html, Properties,
};
use yew_nested_router::Target;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    Index,
}

pub fn switch_app_route(target: AppRoute) -> Html {
    match target {
        AppRoute::Index => html! {<Index/>},
    }
}