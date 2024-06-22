use yew::{function_component, html, Html};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline, Paragraph, CodeBlock
};

#[function_component(GettingStarted)]
pub fn getting_started() -> Html {
    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::None}>
                <Headline>{"Getting Started"}</Headline>
                <Paragraph>{"To get started with Zirv UI, you can install the package using cargo:"}</Paragraph>
                <CodeBlock>{"cargo add zirv_ui"}</CodeBlock>
                <Paragraph>{"After installing the package, you need to add the styling to the index.html. This can be done by adding the following snippet to the head:"}</Paragraph>
                <CodeBlock>{"<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/gh/Glubiz/Zirv-ui@main/style/dist/main.css\">"}</CodeBlock>

            </Container>
        </section>
    }
}
