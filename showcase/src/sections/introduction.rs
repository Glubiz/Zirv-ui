use yew::{function_component, html, Html};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline, Paragraph,
};

#[function_component(IntroductionSection)]
pub fn introduction() -> Html {
    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::None}>
                <Headline>{"Introduction"}</Headline>
                <Paragraph>{"Zirv UI is a component library for Yew, that enables developers to create beautiful websites without the need for CSS"}</Paragraph>
                <Paragraph>{"On this website you will find every thing you need to get started with Zirv UI. In addition to that you will also find the source code on GitHub, feel free to contribute!"}</Paragraph>
            </Container>
        </section>
    }
}
