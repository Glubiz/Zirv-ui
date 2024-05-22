use yew::{classes, function_component, html, Callback, Children, Classes, Html, MouseEvent, Properties};

use crate::{border::{Border, BorderColor, BorderRadius, BorderStyle, BorderWidth}, color::BackgroundColor, font::{FontSize, TextColor}, size::{Height, Width}, spacing::{Margin, Padding}};

#[derive(Properties, Clone, PartialEq, Default)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or(Width::Custom(12))]
    pub width: Width,
    #[prop_or(Height::Custom(4))]
    pub height: Height,
    #[prop_or_default]
    pub border: Border,
    #[prop_or(BorderRadius::Rounded)]
    pub border_radius: BorderRadius,
    #[prop_or_default]
    pub border_color: BorderColor,
    #[prop_or_default]
    pub border_width: BorderWidth,
    #[prop_or_default]
    pub border_style: BorderStyle,
    #[prop_or_default]
    pub padding: Padding,
    #[prop_or_default]
    pub margin: Margin,
    #[prop_or(BackgroundColor::Primary)]
    pub background_color: BackgroundColor,
    #[prop_or(TextColor::TextPrimary)]
    pub font_color: TextColor,
    #[prop_or_default]
    pub font_size: FontSize,

    #[prop_or(None)]
    pub classes: Option<Classes>,
    pub onclick: Callback<MouseEvent>
    
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = classes!(
        &props.width,
        &props.height,
        &props.border,
        &props.border_radius,
        &props.border_color,
        &props.border_width,
        &props.border_style,
        &props.padding,
        &props.margin,
        &props.background_color,
        &props.font_color,
        &props.font_size,
        Some(props.classes.clone())
    );

    html! {
        <button class={classes} onclick={&props.onclick}>
            {props.children.clone()}
        </button>
    }
}