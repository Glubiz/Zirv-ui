use crate::pages::index::Index;

use yew::{
    html, Html,
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