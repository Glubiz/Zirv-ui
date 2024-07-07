use yew::{function_component, html, Html};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline,
};

#[function_component(TextSection)]
pub fn text() -> Html {
    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::None}>
                <Headline>{"Text"}</Headline>
            </Container>
        </section>
    }
}
