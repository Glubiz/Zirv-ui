use yew::{function_component, html, Html, classes};
use zirv_ui::{
    options::{display::Display, size::Width,flex::{FlexAlign, FlexDirection, FlexGap, FlexJustify}},
    Container, Headline, Paragraph, Divider
};

#[function_component(DividerSection)]
pub fn devider() -> Html {
    html! {
        <section class={classes!(&Width::Full)}>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Center} flex_gap={FlexGap::Large} >
                <Headline>{"Divider Component"}</Headline>
                <Divider/>
            </Container>
        </section>
    }
}
