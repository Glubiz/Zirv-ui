//! Image Component
//! 
//! This module provides a customizable image component for the Yew framework. The `Image` component
//! supports various properties such as source URL, alt text, height, width, and additional CSS classes.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html};
//! use zirv_ui::{Image, ImageProps};
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Image src="https://example.com/image.png" alt="Example Image" />
//!     }
//! }
//! ```

use yew::{classes, function_component, html, Classes, Html, Properties};
use crate::options::size::{Height, Width};

/// Properties for the `Image` component.
#[derive(Clone, PartialEq, Default, Properties)]
pub struct ImageProps {
    /// The source URL of the image.
    pub src: String,
    /// The alt text for the image.
    pub alt: String,
    /// The height of the image. Default is `Height::Auto`.
    #[prop_or(Height::Auto)]
    pub height: Height,
    /// The width of the image. Default is `Width::Auto`.
    #[prop_or(Width::Auto)]
    pub width: Width,
    /// Additional CSS classes to apply to the image.
    #[prop_or(None)]
    pub classes: Option<Classes>,
}

/// The `Image` component.
///
/// The `Image` component is used to display an image with customizable properties for
/// source URL, alt text, height, width, and additional CSS classes.
///
/// # Properties
///
/// - `src`: The source URL of the image.
/// - `alt`: The alt text for the image.
/// - `height`: The height of the image. Default is `Height::Auto`.
/// - `width`: The width of the image. Default is `Width::Auto`.
/// - `classes`: Additional CSS classes to apply to the image.
#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let classes = classes!(&props.height, &props.width, &props.classes);

    html! {
        <img src={props.src.clone()} alt={props.alt.clone()} class={classes} />
    }
}
