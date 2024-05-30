use crate::sections::introduction::Introduction;
use yew::prelude::*;
use zirv_ui::{Container, Loader, Style};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <Container>
                <Introduction />
                <Loader style={Style::Spinner} />
                <Loader style={Style::Dots} />
            </Container>
        </>
    }
}
