use crate::pages::{index::IndexPage, getting_started::GettingStartedPage, not_found::NotFoundPage, button::ButtonPage, loader::LoaderPage, table::TablePage, text::TextPage, toast::ToastPage, container::ContainerPage, theme::ThemePage, divider::DividerPage};
use yew::{html, Html};
use yew_router::Routable;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/getting-started")]
    GettingStarted,
    #[at("/button")]
    Button,
    #[at("/loader")]
    Loader,
    #[at("/table")]
    Table,
    #[at("/text")]
    Text,
    #[at("/toast")]
    Toast,
    #[at("/theme")]
    Theme,
    #[at("/container")]
    Container,
    #[at("/divider")]
    Divider,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <IndexPage /> },
        Route::GettingStarted => html! { <GettingStartedPage /> },
        Route::Button => html! { <ButtonPage /> },
        Route::Loader => html! { <LoaderPage /> },
        Route::Table => html! { <TablePage /> },
        Route::Text => html! { <TextPage /> },
        Route::Theme => html! { <ThemePage /> },
        Route::Toast => html! { <ToastPage /> },
        Route::Container => html! { <ContainerPage /> },
        Route::Divider => html! { <DividerPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}
