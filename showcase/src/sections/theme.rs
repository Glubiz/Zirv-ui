use yew::{function_component, html, Html};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline, Paragraph, Subheadline, CodeBlock
};

#[function_component(ThemeSection)]
pub fn theme_section() -> Html {
    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::Large}>
                <Headline>{"Theme Module"}</Headline>

                <Paragraph>{"The Theme module in Zirv UI provides a robust system for styling applications. It includes a Theme struct, utility functions for color manipulation, and a ThemeProvider component for Yew applications."}</Paragraph>

                <Subheadline>{"Theme Struct"}</Subheadline>
                <Paragraph>{"The Theme struct represents a complete theme with various color properties. It includes colors for background, module, text, primary, secondary, tertiary, success, warning, error, and disabled states, along with their dark and light variants."}</Paragraph>

                <Subheadline>{"Creating a Theme"}</Subheadline>
                <Paragraph>{"You can create a theme using the default implementation or by customizing it:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
// Using default theme
let default_theme = Theme::default();

// Customizing theme
let custom_theme = Theme::default()
    .set_background_color("rgb(0,0,0)")
    .set_primary_color("rgb(255,0,0)");
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Theme Properties"}</Subheadline>
                <Paragraph>{"The Theme struct includes the following properties, each with a regular, dark, and light variant:"}</Paragraph>
                <ul>
                    <li>{"background_color"}</li>
                    <li>{"module_color"}</li>
                    <li>{"text_color_primary and text_color_secondary"}</li>
                    <li>{"primary_color"}</li>
                    <li>{"secondary_color"}</li>
                    <li>{"tertiary_color"}</li>
                    <li>{"success_color"}</li>
                    <li>{"warning_color"}</li>
                    <li>{"error_color"}</li>
                    <li>{"disabled_color"}</li>
                </ul>

                <Subheadline>{"Customizing Theme Colors"}</Subheadline>
                <Paragraph>{"The Theme struct provides methods to customize each color property:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
let custom_theme = Theme::default()
    .set_background_color("rgb(37,46,66)")
    .set_primary_color("rgb(44,105,141)")
    .set_success_color("rgb(46,204,113)")
    .set_error_color("rgb(231,76,60)");
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Color Variants"}</Subheadline>
                <Paragraph>{"When setting a color, the Theme automatically generates dark and light variants:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
// This will set primary_color, primary_color_dark, and primary_color_light
.set_primary_color("rgb(44,105,141)")
                    "#}
                    language="Rust"
                />

                <Subheadline>{"ThemeProvider Component"}</Subheadline>
                <Paragraph>{"The ThemeProvider component applies the provided theme to its children by injecting CSS variables:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
use zirv_ui::{Theme, ThemeProvider};

let custom_theme = Theme::default()
    .set_background_color("rgb(0,0,0)")
    .set_primary_color("rgb(255,0,0)");

html! {
    <ThemeProvider theme={custom_theme}>
        <YourApp />
    </ThemeProvider>
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Using Theme Variables"}</Subheadline>
                <Paragraph>{"Once a theme is applied using ThemeProvider, you can use the CSS variables in your styles:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
.your-component {
    background-color: var(--background-color);
    color: var(--text-color-primary);
}

.primary-button {
    background-color: var(--color-primary);
    color: var(--text-color-primary);
}
                    "#}
                    language="CSS"
                />

                <Subheadline>{"Best Practices"}</Subheadline>
                <Paragraph>{"When using the Theme module, consider the following best practices:"}</Paragraph>
                <ul>
                    <li>{"Create a single theme instance for your entire application."}</li>
                    <li>{"Use the ThemeProvider at the root of your application to apply the theme globally."}</li>
                    <li>{"Utilize the CSS variables provided by the theme in your component styles."}</li>
                    <li>{"When customizing colors, ensure they maintain proper contrast for accessibility."}</li>
                    <li>{"Take advantage of the automatically generated dark and light variants for hover and active states."}</li>
                </ul>

                <Paragraph>{"The Theme module in Zirv UI provides a powerful and flexible way to manage the visual styling of your application. By using themes, you can easily maintain consistent colors throughout your UI and quickly apply global style changes."}</Paragraph>
            </Container>
        </section>
    }
}