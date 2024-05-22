use zirv_ui::{Container, Headline, Loader, Style};
use yew::prelude::*;
use zirv_ui::options::color::BackgroundColor;

#[function_component(Index)]
pub fn index() -> Html {

    html! {
        <>
            <Container>
                <Headline>{"Hello, World!"}</Headline>
                <Loader style={Style::Spinner} />
                <Loader style={Style::Dots} />
            </Container>
        </>
    }
}
