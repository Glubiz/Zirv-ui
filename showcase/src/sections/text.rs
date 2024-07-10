use yew::{function_component, html, Html};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline, Paragraph, Subheadline, CodeBlock,
};

#[function_component(TextComponentsSection)]
pub fn text_components_section() -> Html {
    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::Large}>
                <Headline>{"Text Components"}</Headline>

                <Paragraph>{"Zirv UI provides three main text components: Headline, Subheadline, and Paragraph. These components are designed to structure and style text content in your application."}</Paragraph>

                <Subheadline>{"Headline Component"}</Subheadline>
                <Paragraph>{"The Headline component is used for main titles or section headers in your application."}</Paragraph>

                <Subheadline>{"Basic Usage"}</Subheadline>
                <CodeBlock 
                    snippet={r#"
use yew::{function_component, html, Html};
use zirv_ui::Headline;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Headline>{"My Main Title"}</Headline>
    }
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Live Example"}</Subheadline>
                <Headline>{"This is a Headline"}</Headline>

                <Subheadline>{"Subheadline Component"}</Subheadline>
                <Paragraph>{"The Subheadline component is used for subtitles or less prominent headers."}</Paragraph>

                <Subheadline>{"Basic Usage"}</Subheadline>
                <CodeBlock 
                    snippet={r#"
use yew::{function_component, html, Html};
use zirv_ui::Subheadline;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Subheadline>{"My Subtitle"}</Subheadline>
    }
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Live Example"}</Subheadline>
                <Subheadline>{"This is a Subheadline"}</Subheadline>

                <Subheadline>{"Paragraph Component"}</Subheadline>
                <Paragraph>{"The Paragraph component is used for body text and general content."}</Paragraph>

                <Subheadline>{"Basic Usage"}</Subheadline>
                <CodeBlock 
                    snippet={r#"
use yew::{function_component, html, Html};
use zirv_ui::Paragraph;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Paragraph>{"This is a paragraph of text."}</Paragraph>
    }
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Live Example"}</Subheadline>
                <Paragraph>{"This is a Paragraph component. It's used for body text and general content in your application."}</Paragraph>

                <Subheadline>{"Customization"}</Subheadline>
                <Paragraph>{"Each text component may accept properties for customization, such as font size, color, or alignment. Refer to the specific component documentation for available properties."}</Paragraph>

                <Subheadline>{"Example with Custom Properties"}</Subheadline>
                <CodeBlock 
                    snippet={r#"
use yew::{function_component, html, Html};
use zirv_ui::{Headline, options::font::FontSize};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Headline font_size={FontSize::ExtraLarge}>{"Large Headline"}</Headline>
    }
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Best Practices"}</Subheadline>
                <Paragraph>{"When using text components, consider the following best practices:"}</Paragraph>
                <ul>
                    <li>{"Use Headline for main titles, Subheadline for section headers, and Paragraph for body text."}</li>
                    <li>{"Maintain a consistent hierarchy of text styles throughout your application."}</li>
                    <li>{"Ensure proper contrast between text and background colors for readability."}</li>
                    <li>{"Use appropriate font sizes to maintain readability across different devices."}</li>
                    <li>{"Consider using custom properties to align text components with your application's design system."}</li>
                </ul>

                <Paragraph>{"The text components in Zirv UI provide a structured way to present text content in your application, allowing for consistent styling and easy maintenance of your text hierarchy."}</Paragraph>
            </Container>
        </section>
    }
}