//! Container Component
//! 
//! This module provides a customizable container component for the Yew framework. The `Container` component
//! supports various layout and styling options such as width, height, display, flexbox properties, border properties,
//! padding, margin, and background color. It also supports custom classes and can contain child elements.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html, Children};
//! use crate::components::container::{Container, ContainerProps};
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Container>
//!             <p>{"This is inside a container"}</p>
//!         </Container>
//!     }
//! }
//! ```

use yew::{classes, function_component, html, Children, Classes, Html, Properties};

use crate::options::{
    color::BackgroundColor, border::{Border, BorderColor, BorderRadius, BorderStyle, BorderWidth}, display::Display, flex::{FlexAlign, FlexDirection, FlexGrow, FlexJustify, FlexShrink, FlexWrap}, size::{Height, Width}, spacing::{Margin, Padding}
};

/// Properties for the `Container` component.
#[derive(Properties, Clone, PartialEq, Default)]
pub struct ContainerProps {
    /// The content to be rendered inside the container.
    pub children: Children,
    /// The width of the container. Default is `Width::Auto`.
    #[prop_or(Width::Auto)]
    pub width: Width,
    /// The height of the container. Default is `Height::Auto`.
    #[prop_or(Height::Auto)]
    pub height: Height,
    /// The display property of the container.
    #[prop_or_default]
    pub display: Display,
    /// The flex direction property of the container.
    #[prop_or_default]
    pub flex_direction: FlexDirection,
    /// The flex wrap property of the container.
    #[prop_or_default]
    pub flex_wrap: FlexWrap,
    /// The flex grow property of the container.
    #[prop_or_default]
    pub flex_grow: FlexGrow,
    /// The flex shrink property of the container.
    #[prop_or_default]
    pub flex_shrink: FlexShrink,
    /// The flex alignment property of the container.
    #[prop_or_default]
    pub flex_align: FlexAlign,
    /// The flex justify property of the container.
    #[prop_or_default]
    pub flex_justify: FlexJustify,
    /// The border properties of the container.
    #[prop_or_default]
    pub border: Border,
    /// The border radius of the container.
    #[prop_or_default]
    pub border_radius: BorderRadius,
    /// The border color of the container.
    #[prop_or_default]
    pub border_color: BorderColor,
    /// The border width of the container.
    #[prop_or_default]
    pub border_width: BorderWidth,
    /// The border style of the container.
    #[prop_or_default]
    pub border_style: BorderStyle,
    /// The padding inside the container.
    #[prop_or_default]
    pub padding: Padding,
    /// The margin outside the container.
    #[prop_or_default]
    pub margin: Margin,
    /// The background color of the container. Default is `BackgroundColor::Container`.
    #[prop_or(BackgroundColor::Container)]
    pub background_color: BackgroundColor,
    /// Additional CSS classes to apply to the container.
    #[prop_or(None)]
    pub classes: Option<Classes>
}

/// The `Container` component.
///
/// The `Container` component is a versatile layout element that can be customized with various properties for
/// width, height, display, flexbox layout, borders, padding, margin, and background color. It can also accept
/// additional CSS classes and render child elements.
///
/// # Properties
///
/// - `children`: The content to be rendered inside the container.
/// - `width`: The width of the container. Default is `Width::Auto`.
/// - `height`: The height of the container. Default is `Height::Auto`.
/// - `display`: The display property of the container.
/// - `flex_direction`: The flex direction property of the container.
/// - `flex_wrap`: The flex wrap property of the container.
/// - `flex_grow`: The flex grow property of the container.
/// - `flex_shrink`: The flex shrink property of the container.
/// - `flex_align`: The flex alignment property of the container.
/// - `flex_justify`: The flex justify property of the container.
/// - `border`: The border properties of the container.
/// - `border_radius`: The border radius of the container.
/// - `border_color`: The border color of the container.
/// - `border_width`: The border width of the container.
/// - `border_style`: The border style of the container.
/// - `padding`: The padding inside the container.
/// - `margin`: The margin outside the container.
/// - `background_color`: The background color of the container. Default is `BackgroundColor::Container`.
/// - `classes`: Additional CSS classes to apply to the container.
#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let classes = classes!(
        &props.width,
        &props.height,
        &props.display,
        &props.flex_direction,
        &props.flex_wrap,
        &props.flex_grow,
        &props.flex_shrink,
        &props.flex_align,
        &props.flex_justify,
        &props.border,
        &props.border_radius,
        &props.border_color,
        &props.border_width,
        &props.border_style,
        &props.padding,
        &props.margin,
        &props.background_color,
        &props.classes
    );

    html! {
        <div class={classes!(classes)}>
            { props.children.clone() }
        </div>
    }
}
