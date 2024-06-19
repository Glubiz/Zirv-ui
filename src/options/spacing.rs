//! Spacing Enums
//!
//! This module defines the `Padding` and `Margin` enums, which represent various spacing property options for CSS.
//! It also provides an implementation to convert these enums into Yew's `Classes` for CSS styling.
//!
//! # Example
//!
//! ```rust
//! use yew::{html, function_component, Html, Classes, classes};
//! use zirv_ui::options::spacing::{Padding, Margin};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!
//!     html! {
//!         <div class={classes!(&Margin::Medium, &Padding::Medium)}>
//!             {"This div has large padding and medium margin"}
//!         </div>
//!     }
//! }
//! ```

use yew::{classes, Classes};

/// Enum representing the padding property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Padding {
    /// No padding.
    None,
    /// Small padding.
    Small,
    /// Medium padding. This is the default.
    #[default]
    Medium,
    /// Large padding.
    Large,
}

impl From<&Padding> for Classes {
    /// Converts a `Padding` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::spacing::Padding;
    ///
    /// let padding_class: Classes = (&Padding::Large).into();
    /// ```
    fn from(padding: &Padding) -> Self {
        match padding {
            Padding::None => classes!("padding-none"),
            Padding::Small => classes!("padding-small"),
            Padding::Medium => classes!("padding-medium"),
            Padding::Large => classes!("padding-large"),
        }
    }
}

/// Enum representing the margin property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Margin {
    /// No margin.
    None,
    /// Small margin. This is the default.
    #[default]
    Small,
    /// Medium margin.
    Medium,
    /// Large margin.
    Large,
}

impl From<&Margin> for Classes {
    /// Converts a `Margin` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::spacing::Margin;
    ///
    /// let margin_class: Classes = (&Margin::Medium).into();
    /// ```
    fn from(margin: &Margin) -> Self {
        match margin {
            Margin::None => classes!("margin-none"),
            Margin::Small => classes!("margin-small"),
            Margin::Medium => classes!("margin-medium"),
            Margin::Large => classes!("margin-large"),
        }
    }
}
