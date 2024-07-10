use yew::{function_component, html, Html, Callback};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline, Paragraph, CodeBlock, Subheadline, Button,
    border::{BorderRadius, BorderColor, BorderWidth, BorderStyle},
    color::BackgroundColor,
    font::{FontSize, TextColor},
    size::{Width, Height, CustomType},
    spacing::Padding,
};

#[function_component(ButtonSection)]
pub fn button_section() -> Html {
    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::Large}>
                <Headline>{"Button Component"}</Headline>

                <Paragraph>{"The Button component is a customizable button for use in Yew applications using Zirv UI. It offers a wide range of styling options and supports an onclick callback."}</Paragraph>

                <Subheadline>{"Basic Usage"}</Subheadline>
                <Paragraph>{"Here's a simple example of how to use the Button component:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
use yew::{function_component, html, Callback};
use zirv_ui::Button;

#[function_component(App)]
fn app() -> Html {
    let onclick = Callback::from(|_| log::info!("Button clicked!"));

    html! {
        <Button onclick={onclick}>
            {"Click Me"}
        </Button>
    }
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Properties"}</Subheadline>
                <Paragraph>{"The Button component accepts the following properties:"}</Paragraph>
                <ul>
                    <li>{"children: The content to be rendered inside the button."}</li>
                    <li>{"width: The width of the button. Default is Width::Custom(12, CustomType::Fixed)."}</li>
                    <li>{"height: The height of the button. Default is Height::Custom(4, CustomType::Fixed)."}</li>
                    <li>{"border: The border properties of the button."}</li>
                    <li>{"border_radius: The border radius of the button. Default is BorderRadius::Rounded."}</li>
                    <li>{"border_color: The border color of the button."}</li>
                    <li>{"border_width: The border width of the button. Options are None, Small, Medium, Large."}</li>
                    <li>{"border_style: The border style of the button."}</li>
                    <li>{"padding: The padding inside the button."}</li>
                    <li>{"margin: The margin outside the button."}</li>
                    <li>{"background_color: The background color of the button. Default is BackgroundColor::Primary."}</li>
                    <li>{"font_color: The font color of the button text. Default is TextColor::TextPrimary."}</li>
                    <li>{"font_size: The font size of the button text."}</li>
                    <li>{"classes: Additional CSS classes to apply to the button."}</li>
                    <li>{"onclick: Callback to be executed when the button is clicked."}</li>
                </ul>

                <Subheadline>{"Customization Examples"}</Subheadline>
                
                <Paragraph>{"1. Customizing Size:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Button 
    width={Width::Custom(20, CustomType::Fixed)} 
    height={Height::Custom(6, CustomType::Fixed)} 
    onclick={onclick}
>
    {"Large Button"}
</Button>
                    "#}
                    language="Rust"
                />

                <Paragraph>{"2. Customizing Colors:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Button 
    background_color={BackgroundColor::Secondary} 
    font_color={TextColor::TextSecondary} 
    onclick={onclick}
>
    {"Colored Button"}
</Button>
                    "#}
                    language="Rust"
                />

                <Paragraph>{"3. Customizing Border:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Button 
    border_radius={BorderRadius::None}
    border_color={BorderColor::Primary}
    border_width={BorderWidth::Large}
    border_style={BorderStyle::Solid}
    onclick={onclick}
>
    {"Bordered Button"}
</Button>
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Hover Effect"}</Subheadline>
                <Paragraph>{"The Button component includes a built-in hover effect that darkens the background color when the mouse is over the button."}</Paragraph>

                <Subheadline>{"Live Example"}</Subheadline>
                <Paragraph>{"Here's a live example of the Button component with various customizations:"}</Paragraph>

                <Button 
                    onclick={Callback::from(|_| log::info!("Default button clicked"))}
                >
                    {"Default Button"}
                </Button>

                <Button 
                    width={Width::Custom(20, CustomType::Fixed)}
                    height={Height::Custom(6, CustomType::Fixed)}
                    font_color={TextColor::TextSecondary}
                    font_size={FontSize::Large}
                    onclick={Callback::from(|_| log::info!("Custom button clicked"))}
                >
                    {"Custom Button"}
                </Button>

                <Button 
                    width={Width::Custom(20, CustomType::Fixed)}
                    height={Height::Custom(6, CustomType::Fixed)}
                    border_radius={BorderRadius::None}
                    border_color={BorderColor::Secondary}
                    border_width={BorderWidth::Large}
                    border_style={BorderStyle::Solid}
                    padding={Padding::Medium}
                    onclick={Callback::from(|_| log::info!("Bordered button clicked"))}
                >
                    {"Bordered Button"}
                </Button>

                <Subheadline>{"Best Practices"}</Subheadline>
                <Paragraph>{"When using the Button component, consider the following best practices:"}</Paragraph>
                <ul>
                    <li>{"Use consistent styling across your application for a uniform look and feel."}</li>
                    <li>{"Choose appropriate colors that contrast well with your text for better readability."}</li>
                    <li>{"Provide meaningful text or icons inside your buttons to clearly convey their purpose."}</li>
                    <li>{"Use the onclick callback to handle user interactions and perform necessary actions."}</li>
                </ul>

                <Paragraph>{"The Button component is a versatile and customizable element that can be used throughout your Zirv UI application to create interactive and visually appealing user interfaces."}</Paragraph>
            </Container>
        </section>
    }
}