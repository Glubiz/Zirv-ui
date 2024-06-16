//! Overflow Enum
//! 
//! This module defines the `Overflow` enum, which represents various overflow property options for CSS.
//! It also provides an implementation to convert `Overflow` into Yew's `Classes` for CSS styling.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html, function_component, Html};
//! use crate::options::overflow::Overflow;
//! use yew::Classes;
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     let overflow_class: Classes = (&Overflow::Scroll).into();
//! 
//!     html! {
//!         <div class={overflow_class}>
//!             {"This div has scroll overflow"}
//!         </div>
//!     }
//! }
//! ```

use yew::{classes, Classes};

/// Enum representing the overflow property options.
#[derive(Clone, PartialEq, Debug, Default)]
pub enum Overflow {
    /// Overflow is visible.
    Visible,
    /// Overflow is hidden.
    Hidden,
    /// Overflow is scrollable.
    Scroll,
    /// Overflow is set to auto. This is the default.
    #[default]
    Auto,
}

impl From<&Overflow> for Classes {
    /// Converts an `Overflow` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::overflow::Overflow;
    /// 
    /// let overflow_class: Classes = (&Overflow::Hidden).into();
    /// ```
    fn from(overflow: &Overflow) -> Self {
        match overflow {
            Overflow::Visible => classes!("overflow-visible"),
            Overflow::Hidden => classes!("overflow-hidden"),
            Overflow::Scroll => classes!("overflow-scroll"),
            Overflow::Auto => classes!("overflow-auto"),
        }
    }
}
