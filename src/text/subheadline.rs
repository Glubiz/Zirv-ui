//! Subheadline Component
//!
//! This module provides a customizable subheadline component for the Yew framework. The
//! `Subheadline` component supports various styling options for font size, weight, style, and
//! family, as well as additional CSS classes.
//!
//! # Example
//!
//! ```rust
//! use yew::{
//!     function_component,
//!     html,
//!     Html,
//! };
//! use zirv_ui::{
//!     Subheadline,
//!     SubheadlineProps,
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Subheadline>
//!             {"This is a customizable subheadline"}
//!         </Subheadline>
//!     }
//! }
//! ```

use yew::{
    classes,
    function_component,
    html,
    Children,
    Classes,
    Html,
    Properties,
};

use crate::options::font::{
    FontFamily,
    FontSize,
    FontStyle,
    FontWeight,
};

/// Properties for the `Subheadline` component.
#[derive(Properties, Clone, PartialEq)]
pub struct SubheadlineProps {
    /// The children (content) of the subheadline.
    pub children: Children,
    /// The font size of the subheadline. Default is `FontSize::None`.
    #[prop_or_default]
    pub size: FontSize,
    /// The font weight of the subheadline. Default is `FontWeight::Normal`.
    #[prop_or_default]
    pub weight: FontWeight,
    /// The font style of the subheadline. Default is `FontStyle::Normal`.
    #[prop_or_default]
    pub style: FontStyle,
    /// The font family of the subheadline. Default is `FontFamily::Arial`.
    #[prop_or_default]
    pub family: FontFamily,
    /// Additional CSS classes to apply to the subheadline.
    #[prop_or(None)]
    pub classes: Option<Classes>,
}

/// The `Subheadline` component.
///
/// The `Subheadline` component is used to display a subheadline (typically an `<h3>` element) with
/// customizable properties for font size, weight, style, and family, as well as additional CSS
/// classes.
///
/// # Properties
///
/// - `children`: The children (content) of the subheadline.
/// - `size`: The font size of the subheadline. Default is `FontSize::None`.
/// - `weight`: The font weight of the subheadline. Default is `FontWeight::Normal`.
/// - `style`: The font style of the subheadline. Default is `FontStyle::Normal`.
/// - `family`: The font family of the subheadline. Default is `FontFamily::Arial`.
/// - `classes`: Additional CSS classes to apply to the subheadline.
#[function_component(Subheadline)]
pub fn subheadline(props: &SubheadlineProps) -> Html {
    let classes = classes!(&props.size, &props.weight, &props.style, &props.family, &props.classes);

    html! {
        <h3 class={classes!(classes)}>
            {props.children.clone()}
        </h3>
    }
}
