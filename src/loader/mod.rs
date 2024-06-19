//! Loader Component
//!
//! This module provides a customizable loader component for the Yew framework. The `Loader`
//! component supports two styles: `Dots` and `Spinner`. It is used to display a loading animation
//! in your application.
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
//!     Loader,
//!     LoaderProps,
//!     Style,
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <>
//!             <Loader style={Style::Dots} />
//!             <Loader style={Style::Spinner} />
//!         </>
//!     }
//! }
//! ```

use yew::{
    classes,
    function_component,
    html,
    Html,
    Properties,
};

/// Enum representing the style of the loader.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Style {
    /// Dots loader style.
    Dots,
    /// Spinner loader style.
    #[default]
    Spinner,
}

/// Properties for the `Loader` component.
#[derive(Properties, Clone, PartialEq)]
pub struct LoaderProps {
    /// The style of the loader. Default is `Style::Spinner`.
    #[prop_or_default]
    pub style: Style,
}

/// The `Loader` component.
///
/// The `Loader` component is used to display a loading animation with customizable styles.
/// It supports two styles: `Dots` and `Spinner`.
///
/// # Properties
///
/// - `style`: The style of the loader. Default is `Style::Spinner`.
#[function_component(Loader)]
pub fn loader(props: &LoaderProps) -> Html {
    match props.style {
        Style::Dots => html! {
            <div class={classes!("loader-dots")}>
                <div class={classes!("dot")}></div>
                <div class={classes!("dot")}></div>
                <div class={classes!("dot")}></div>
            </div>
        },
        Style::Spinner => html! {
            <div class={classes!("loader-spinner")}></div>
        },
    }
}
