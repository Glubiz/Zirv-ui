//! Flex Component
//!
//! This module provides a flexible container component for the Yew framework. The `Flex` component
//! supports various flexbox layout options such as direction, wrap, alignment, justification, grow,
//! and shrink. It can also accept custom classes and render child elements.

use yew::{
    classes,
    function_component,
    html,
    Children,
    Classes,
    Html,
    Properties,
};

use crate::options::{
    display::Display,
    flex::{
        FlexAlign,
        FlexDirection,
        FlexGrow,
        FlexJustify,
        FlexShrink,
        FlexWrap,
    },
    size::{
        Height,
        Width,
    },
};

/// Properties for the `Flex` component.
#[derive(Properties, Clone, PartialEq)]
pub struct FlexProps {
    /// The content to be rendered inside the flex container.
    pub children: Children,
    /// The flex wrap property of the container.
    #[prop_or_default]
    pub wrap: FlexWrap,
    /// The flex alignment property of the container.
    #[prop_or_default]
    pub align: FlexAlign,
    /// The flex justify property of the container.
    #[prop_or_default]
    pub justify: FlexJustify,
    /// The flex grow property of the container.
    #[prop_or_default]
    pub grow: FlexGrow,
    /// The flex shrink property of the container.
    #[prop_or_default]
    pub shrink: FlexShrink,
    /// The flex direction property of the container.
    #[prop_or_default]
    pub direction: FlexDirection,
    /// The height of the container.
    #[prop_or_default]
    pub height: Height,
    /// The width of the container.
    #[prop_or_default]
    pub width: Width,
    /// Additional CSS classes to apply to the container.
    pub classes: Option<Classes>,
}

/// The `Flex` component.
///
/// The `Flex` component is a versatile layout element that can be customized with various
/// properties for flexbox layout such as direction, wrap, alignment, justification, grow, and
/// shrink. It can also accept additional CSS classes and render child elements.
///
/// # Properties
///
/// - `children`: The content to be rendered inside the flex container.
/// - `wrap`: The flex wrap property of the container.
/// - `align`: The flex alignment property of the container.
/// - `justify`: The flex justify property of the container.
/// - `grow`: The flex grow property of the container.
/// - `shrink`: The flex shrink property of the container.
/// - `direction`: The flex direction property of the container.
/// - `height`: The height of the container.
/// - `width`: The width of the container.
/// - `classes`: Additional CSS classes to apply to the container.
#[function_component(Flex)]
pub fn flex(props: &FlexProps) -> Html {
    html! {
        <div class={classes!(&Display::Flex, &props.direction, &props.align, &props.justify, Some(props.classes.clone()))}>
            {props.children.clone()}
        </div>
    }
}
