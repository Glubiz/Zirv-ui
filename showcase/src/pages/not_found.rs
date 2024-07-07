use yew::prelude::*;
use zirv_ui::{options::flex::{FlexDirection, FlexJustify, FlexAlign}, Container, Headline, Paragraph};

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column} flex_justify={FlexJustify::Center} flex_align={FlexAlign::Center}>
                <Headline>{"404"}</Headline>
                <Paragraph>{"The page you are looking for does not exist"}</Paragraph>
            </Container>
        </div>
    }
}
