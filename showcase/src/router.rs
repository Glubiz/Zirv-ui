use crate::pages::{index::Index, getting_started::GettingStarted, not_found::NotFound};
use yew::{html, Html};
use yew_router::Routable;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/getting-started")]
    GettingStarted,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },
        Route::GettingStarted => html! { <GettingStarted /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
