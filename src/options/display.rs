//! Display Enum
//! 
//! This module defines the `Display` enum, which represents various display property options for CSS.
//! It also provides an implementation to convert `Display` into Yew's `Classes` for CSS styling.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html, function_component, Html};
//! use crate::options::display::Display;
//! use yew::Classes;
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     let display_class: Classes = (&Display::Flex).into();
//! 
//!     html! {
//!         <div class={display_class}>
//!             {"This div has a flex display"}
//!         </div>
//!     }
//! }
//! ```

use yew::{classes, Classes};

/// Enum representing the display property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Display {
    /// No display.
    None,
    /// Block display.
    Block,
    /// Inline display.
    Inline,
    /// Inline-block display.
    InlineBlock,
    /// Flex display. This is the default.
    #[default]
    Flex,
    /// Grid display.
    Grid,
}

impl From<&Display> for Classes {
    /// Converts a `Display` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::display::Display;
    /// 
    /// let display_class: Classes = (&Display::Flex).into();
    /// ```
    fn from(display: &Display) -> Self {
        match display {
            Display::None => classes!("display-none"),
            Display::Block => classes!("display-block"),
            Display::Inline => classes!("display-inline"),
            Display::InlineBlock => classes!("display-inline-block"),
            Display::Flex => classes!("display-flex"),
            Display::Grid => classes!("display-grid"),
        }
    }
}
