//! Headline Component
//! 
//! This module provides a customizable headline component for the Yew framework. The `Headline` component
//! supports various styling options for font size, weight, style, and family, as well as additional CSS classes.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html, function_component, Html};
//! use crate::components::headline::{Headline, HeadlineProps};
//! use crate::options::font::{FontSize, FontWeight, FontStyle, FontFamily};
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Headline size={FontSize::Large} weight={FontWeight::Bold} style={FontStyle::Italic} family={FontFamily::Helvetica}>
//!             {"This is a customizable headline"}
//!         </Headline>
//!     }
//! }
//! ```

use yew::{classes, function_component, html, Children, Classes, Html, Properties};
use crate::options::font::{FontSize, FontWeight, FontStyle, FontFamily};

/// Properties for the `Headline` component.
#[derive(Properties, Clone, PartialEq)]
pub struct HeadlineProps {
    /// The children (content) of the headline.
    pub children: Children,
    /// The font size of the headline. Default is `FontSize::None`.
    #[prop_or_default]
    pub size: FontSize,
    /// The font weight of the headline. Default is `FontWeight::Normal`.
    #[prop_or_default]
    pub weight: FontWeight,
    /// The font style of the headline. Default is `FontStyle::Normal`.
    #[prop_or_default]
    pub style: FontStyle,
    /// The font family of the headline. Default is `FontFamily::Arial`.
    #[prop_or_default]
    pub family: FontFamily,
    /// Additional CSS classes to apply to the headline.
    #[prop_or(None)]
    pub classes: Option<Classes>,
}

/// The `Headline` component.
///
/// The `Headline` component is used to display a headline (typically an `<h1>` element) with customizable
/// properties for font size, weight, style, and family, as well as additional CSS classes.
///
/// # Properties
///
/// - `children`: The children (content) of the headline.
/// - `size`: The font size of the headline. Default is `FontSize::None`.
/// - `weight`: The font weight of the headline. Default is `FontWeight::Normal`.
/// - `style`: The font style of the headline. Default is `FontStyle::Normal`.
/// - `family`: The font family of the headline. Default is `FontFamily::Arial`.
/// - `classes`: Additional CSS classes to apply to the headline.
#[function_component(Headline)]
pub fn headline(props: &HeadlineProps) -> Html {
    let classes = classes!(&props.size, &props.weight, &props.style, &props.family, &props.classes);

    html! {
        <h1 class={classes!(classes)}>
            {props.children.clone()}
        </h1>
    }
}
