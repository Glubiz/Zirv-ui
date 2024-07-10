use yew::{function_component, html, Html};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline, Paragraph, CodeBlock, Subheadline, Loader, Style,
};

#[function_component(LoaderSection)]
pub fn loader_section() -> Html {
    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::Large}>
                <Headline>{"Loader Component"}</Headline>

                <Paragraph>{"The Loader component is a customizable loading animation for use in Yew applications using Zirv UI. It offers two styles: Dots and Spinner."}</Paragraph>

                <Subheadline>{"Basic Usage"}</Subheadline>
                <Paragraph>{"Here's a simple example of how to use the Loader component:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
use yew::{function_component, html, Html};
use zirv_ui::{Loader, Style};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Loader style={Style::Dots} />
            <Loader style={Style::Spinner} />
        </>
    }
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Properties"}</Subheadline>
                <Paragraph>{"The Loader component accepts the following property:"}</Paragraph>
                <ul>
                    <li>{"style: The style of the loader. Options are Style::Dots and Style::Spinner. Default is Style::Spinner."}</li>
                </ul>

                <Subheadline>{"Styles"}</Subheadline>
                <Paragraph>{"The Loader component supports two styles:"}</Paragraph>
                <ul>
                    <li>{"Style::Dots: Displays a loading animation with three dots."}</li>
                    <li>{"Style::Spinner: Displays a spinning loader animation."}</li>
                </ul>

                <Subheadline>{"Customization Examples"}</Subheadline>
                
                <Paragraph>{"1. Dots Loader:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Loader style={Style::Dots} />
                    "#}
                    language="Rust"
                />

                <Paragraph>{"2. Spinner Loader:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Loader style={Style::Spinner} />
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Live Examples"}</Subheadline>
                <Paragraph>{"Here are live examples of the Loader component with both styles:"}</Paragraph>

                <div>
                    <Paragraph>{"Dots Loader:"}</Paragraph>
                    <Loader style={Style::Dots} />
                </div>

                <div>
                    <Paragraph>{"Spinner Loader:"}</Paragraph>
                    <Loader style={Style::Spinner} />
                </div>

                <Subheadline>{"Implementation Details"}</Subheadline>
                <Paragraph>{"The Loader component is implemented using CSS classes for styling. The Dots style uses three separate div elements, while the Spinner style uses a single div element with CSS animations."}</Paragraph>

                <Subheadline>{"Best Practices"}</Subheadline>
                <Paragraph>{"When using the Loader component, consider the following best practices:"}</Paragraph>
                <ul>
                    <li>{"Use loaders to indicate that content is being loaded or an action is in progress."}</li>
                    <li>{"Choose the style that best fits your application's design language."}</li>
                    <li>{"Ensure the loader is visible and contrasts well with the background."}</li>
                    <li>{"Consider using the loader in conjunction with skeleton screens for a better user experience during longer loading times."}</li>
                </ul>

                <Paragraph>{"The Loader component is a simple yet effective way to provide visual feedback to users during asynchronous operations in your Zirv UI application."}</Paragraph>
            </Container>
        </section>
    }
}