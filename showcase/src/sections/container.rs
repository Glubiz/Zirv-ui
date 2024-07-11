use yew::{function_component, html, Html};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap, FlexJustify},
    Container, Headline, Paragraph, CodeBlock, Subheadline,
    options::size::{Width, Height, CustomType},
    options::display::Display,
    options::border::BorderRadius,
    options::color::BackgroundColor,
    options::spacing::Padding,
};

#[function_component(ContainerSection)]
pub fn container_section() -> Html {
    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::Large}>
                <Headline>{"Container Component"}</Headline>

                <Paragraph>{"The Container component is a versatile layout element for use in Yew applications using Zirv UI. It offers a wide range of customization options for layout, sizing, and styling."}</Paragraph>

                <Subheadline>{"Basic Usage"}</Subheadline>
                <Paragraph>{"Here's a simple example of how to use the Container component:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
use yew::{function_component, html, Html};
use zirv_ui::Container;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Container>
            <p>{"This is inside a container"}</p>
        </Container>
    }
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Properties"}</Subheadline>
                <Paragraph>{"The Container component accepts numerous properties for customization:"}</Paragraph>
                <ul>
                    <li>{"children: The content to be rendered inside the container."}</li>
                    <li>{"width: Width - The width of the container. Default is Width::Auto."}</li>
                    <li>{"height: Height - The height of the container. Default is Height::Auto."}</li>
                    <li>{"display: Display - The display property of the container."}</li>
                    <li>{"flex_direction: FlexDirection - The flex direction of the container."}</li>
                    <li>{"flex_wrap: FlexWrap - The flex wrap property of the container."}</li>
                    <li>{"flex_grow: FlexGrow - The flex grow property of the container."}</li>
                    <li>{"flex_shrink: FlexShrink - The flex shrink property of the container."}</li>
                    <li>{"flex_align: FlexAlign - The flex alignment property of the container."}</li>
                    <li>{"flex_justify: FlexJustify - The flex justify property of the container."}</li>
                    <li>{"flex_gap: FlexGap - The flex gap property of the container."}</li>
                    <li>{"border: Border - The border properties of the container."}</li>
                    <li>{"border_radius: BorderRadius - The border radius of the container."}</li>
                    <li>{"border_color: BorderColor - The border color of the container."}</li>
                    <li>{"border_width: BorderWidth - The border width of the container."}</li>
                    <li>{"border_style: BorderStyle - The border style of the container."}</li>
                    <li>{"padding: Padding - The padding inside the container."}</li>
                    <li>{"margin: Margin - The margin outside the container."}</li>
                    <li>{"background_color: BackgroundColor - The background color of the container. Default is BackgroundColor::Container."}</li>
                    <li>{"classes: Option<Classes> - Additional CSS classes to apply to the container."}</li>
                </ul>

                <Subheadline>{"Customization Examples"}</Subheadline>
                
                <Paragraph>{"1. Customizing size and background:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Container 
    width={Width::Full}
    height={Height::Screen}
    background_color={BackgroundColor::Primary}
>
    {"Full-width, full-height container"}
</Container>
                    "#}
                    language="Rust"
                />

                <Paragraph>{"2. Using flexbox properties:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Container 
    display={Display::Flex}
    flex_direction={FlexDirection::Column}
    flex_align={FlexAlign::Center}
    flex_justify={FlexJustify::SpaceBetween}
>
    <div>{"Top"}</div>
    <div>{"Middle"}</div>
    <div>{"Bottom"}</div>
</Container>
                    "#}
                    language="Rust"
                />

                <Paragraph>{"3. Adding border and padding:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Container 
    border_width={BorderWidth::Medium}
    border_style={BorderStyle::Solid}
    border_color={BorderColor::Primary}
    border_radius={BorderRadius::Large}
    padding={Padding::Large}
>
    {"Container with border and padding"}
</Container>
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Live Example"}</Subheadline>
                <Paragraph>{"Here's a live example of the Container component with various properties:"}</Paragraph>

                <Container 
                    width={Width::Full}
                    height={Height::Custom(36, CustomType::Fixed)}
                    display={Display::Flex}
                    flex_direction={FlexDirection::Column}
                    flex_align={FlexAlign::Center}
                    flex_justify={FlexJustify::Center}
                    background_color={BackgroundColor::Secondary}
                    border_radius={BorderRadius::Medium}
                    padding={Padding::Medium}
                >
                    <Paragraph>{"This is a customized Container"}</Paragraph>
                    <Paragraph>{"It demonstrates various properties"}</Paragraph>
                </Container>

                <Subheadline>{"Implementation Details"}</Subheadline>
                <Paragraph>{"The Container component uses Yew's classes macro to apply styling based on the provided properties. It generates a div element with the appropriate classes and renders its children inside."}</Paragraph>

                <Subheadline>{"Best Practices"}</Subheadline>
                <Paragraph>{"When using the Container component, consider the following best practices:"}</Paragraph>
                <ul>
                    <li>{"Use containers to create consistent layout structures across your application."}</li>
                    <li>{"Leverage flexbox properties for responsive layouts."}</li>
                    <li>{"Combine multiple containers with different properties to create complex layouts."}</li>
                    <li>{"Use the background_color property to create visual separation between sections."}</li>
                    <li>{"Utilize padding and margin properties to control spacing within and around containers."}</li>
                    <li>{"Consider accessibility when choosing colors and contrast for your containers."}</li>
                </ul>

                <Paragraph>{"The Container component provides a flexible foundation for creating layouts in your Zirv UI application, with extensive customization options to fit various design needs."}</Paragraph>
            </Container>
        </section>
    }
}