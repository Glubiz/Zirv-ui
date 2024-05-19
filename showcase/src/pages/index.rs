use zirv_ui::{Container, Headline};
use yew::prelude::*;
use zirv_ui::options::color::BackgroundColor;

#[function_component(Index)]
pub fn index() -> Html {

    html! {
        <>
            <Container>
                <Headline>{"Hello, World!"}</Headline>
            </Container>
        </>
    }
}
