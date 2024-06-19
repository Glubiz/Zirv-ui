//! Button Component
//! 
//! This module provides a customizable button component for the Yew framework. The `Button` component
//! allows for various styling options such as width, height, border properties, padding, margin, background color,
//! font color, and font size. It also supports custom classes and an `onclick` callback.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html, Callback};
//! use zirv_ui::{Button, ButtonProps};
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     let onclick = Callback::from(|_| log::info!("Button clicked!"));
//! 
//!     html! {
//!         <Button onclick={onclick}>
//!             {"Click Me"}
//!         </Button>
//!     }
//! }
//! ```

use yew::{classes, function_component, html, Callback, Children, Classes, Html, MouseEvent, Properties};
use crate::{border::{Border, BorderColor, BorderRadius, BorderStyle, BorderWidth}, color::BackgroundColor, font::{FontSize, TextColor}, size::{CustomType, Height, Width}, spacing::{Margin, Padding}};

/// Properties for the `Button` component.
#[derive(Properties, Clone, PartialEq, Default)]
pub struct ButtonProps {
    /// The content to be rendered inside the button.
    pub children: Children,
    /// The width of the button. Default is `Width::Custom(12, CustomType::Fixed)`.
    #[prop_or(Width::Custom(12, CustomType::Fixed))]
    pub width: Width,
    /// The height of the button. Default is `Height::Custom(4, CustomType::Fixed)`.
    #[prop_or(Height::Custom(4, CustomType::Fixed))]
    pub height: Height,
    /// The border properties of the button.
    #[prop_or_default]
    pub border: Border,
    /// The border radius of the button. Default is `BorderRadius::Rounded`.
    #[prop_or(BorderRadius::Rounded)]
    pub border_radius: BorderRadius,
    /// The border color of the button.
    #[prop_or_default]
    pub border_color: BorderColor,
    /// The border width of the button.
    #[prop_or_default]
    pub border_width: BorderWidth,
    /// The border style of the button.
    #[prop_or_default]
    pub border_style: BorderStyle,
    /// The padding inside the button.
    #[prop_or_default]
    pub padding: Padding,
    /// The margin outside the button.
    #[prop_or_default]
    pub margin: Margin,
    /// The background color of the button. Default is `BackgroundColor::Primary`.
    #[prop_or(BackgroundColor::Primary)]
    pub background_color: BackgroundColor,
    /// The font color of the button text. Default is `TextColor::TextPrimary`.
    #[prop_or(TextColor::TextPrimary)]
    pub font_color: TextColor,
    /// The font size of the button text.
    #[prop_or_default]
    pub font_size: FontSize,
    /// Additional CSS classes to apply to the button.
    #[prop_or(None)]
    pub classes: Option<Classes>,
    /// Callback to be executed when the button is clicked.
    pub onclick: Callback<MouseEvent>
}

/// The `Button` component.
///
/// # Properties
///
/// - `children`: The content to be rendered inside the button.
/// - `width`: The width of the button. Default is `Width::Custom(12)`.
/// - `height`: The height of the button. Default is `Height::Custom(4)`.
/// - `border`: The border properties of the button.
/// - `border_radius`: The border radius of the button. Default is `BorderRadius::Rounded`.
/// - `border_color`: The border color of the button.
/// - `border_width`: The border width of the button.
/// - `border_style`: The border style of the button.
/// - `padding`: The padding inside the button.
/// - `margin`: The margin outside the button.
/// - `background_color`: The background color of the button. Default is `BackgroundColor::Primary`.
/// - `font_color`: The font color of the button text. Default is `TextColor::TextPrimary`.
/// - `font_size`: The font size of the button text.
/// - `classes`: Additional CSS classes to apply to the button.
/// - `onclick`: Callback to be executed when the button is clicked.
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
