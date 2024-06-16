//! Paragraph Component
//! 
//! This module provides a customizable paragraph component for the Yew framework. The `Paragraph` component
//! supports various styling options for font size, weight, style, and family, as well as additional CSS classes.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html, function_component, Html};
//! use crate::components::paragraph::{Paragraph, ParagraphProps};
//! use crate::options::font::{FontSize, FontWeight, FontStyle, FontFamily};
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Paragraph size={FontSize::Medium} weight={FontWeight::Normal} style={FontStyle::Italic} family={FontFamily::Arial}>
//!             {"This is a customizable paragraph"}
//!         </Paragraph>
//!     }
//! }
//! ```

use yew::{classes, function_component, html, Children, Classes, Html, Properties};
use crate::options::font::{FontFamily, FontSize, FontStyle, FontWeight};

/// Properties for the `Paragraph` component.
#[derive(Properties, Clone, PartialEq)]
pub struct ParagraphProps {
    /// The children (content) of the paragraph.
    pub children: Children,
    /// The font size of the paragraph. Default is `FontSize::None`.
    #[prop_or_default]
    pub size: FontSize,
    /// The font weight of the paragraph. Default is `FontWeight::Normal`.
    #[prop_or_default]
    pub weight: FontWeight,
    /// The font style of the paragraph. Default is `FontStyle::Normal`.
    #[prop_or_default]
    pub style: FontStyle,
    /// The font family of the paragraph. Default is `FontFamily::Arial`.
    #[prop_or_default]
    pub family: FontFamily,
    /// Additional CSS classes to apply to the paragraph.
    #[prop_or(None)]
    pub classes: Option<Classes>,
}

/// The `Paragraph` component.
///
/// The `Paragraph` component is used to display a paragraph (`<p>` element) with customizable
/// properties for font size, weight, style, and family, as well as additional CSS classes.
///
/// # Properties
///
/// - `children`: The children (content) of the paragraph.
/// - `size`: The font size of the paragraph. Default is `FontSize::None`.
/// - `weight`: The font weight of the paragraph. Default is `FontWeight::Normal`.
/// - `style`: The font style of the paragraph. Default is `FontStyle::Normal`.
/// - `family`: The font family of the paragraph. Default is `FontFamily::Arial`.
/// - `classes`: Additional CSS classes to apply to the paragraph.
#[function_component(Paragraph)]
pub fn paragraph(props: &ParagraphProps) -> Html {
    let classes = classes!(&props.size, &props.weight, &props.style, &props.family, &props.classes);

    html! {
        <p class={classes!(classes)}>
            {props.children.clone()}
        </p>
    }
}
