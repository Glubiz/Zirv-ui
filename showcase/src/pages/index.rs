use crate::sections::introduction::Introduction;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container, Loader, Style};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <Introduction />
                <Loader style={Style::Spinner} />
                <Loader style={Style::Dots} />
            </Container>
        </div>
    }
}
