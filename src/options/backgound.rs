//! BackgroundColor Enum
//!
//! This module defines the `BackgroundColor` enum, which represents various background color
//! options. It also provides an implementation to convert `BackgroundColor` into Yew's `Classes`
//! for CSS styling.
//!
//! # Example
//!
//! ```rust
//! use yew::{
//!     function_component,
//!     html,
//!     Classes,
//!     Html,
//! };
//! use zirv_ui::options::color::BackgroundColor;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let background_class: Classes = (&BackgroundColor::Primary).into();
//!
//!     html! {
//!         <div class={background_class}>
//!             {"This div has a primary background color"}
//!         </div>
//!     }
//! }
//! ```

use yew::{
    classes,
    Classes,
};

/// Enum representing the background color options.
pub enum BackgroundColor {
    /// No background color.
    None,
    /// Primary background color.
    Primary,
    /// Secondary background color.
    Secondary,
    /// Tertiary background color.
    Tertiary,
    /// Success background color.
    Success,
    /// Warning background color.
    Warning,
    /// Error background color.
    Error,
}

impl From<&BackgroundColor> for Classes {
    /// Converts a `BackgroundColor` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::color::BackgroundColor;
    ///
    /// let background_class: Classes = (&BackgroundColor::Primary).into();
    /// ```
    fn from(background_color: &BackgroundColor) -> Self {
        match background_color {
            BackgroundColor::None => classes!("background-color-none"),
            BackgroundColor::Primary => classes!("background-color-primary"),
            BackgroundColor::Secondary => classes!("background-color-secondary"),
            BackgroundColor::Tertiary => classes!("background-color-tertiary"),
            BackgroundColor::Success => classes!("background-color-success"),
            BackgroundColor::Warning => classes!("background-color-warning"),
            BackgroundColor::Error => classes!("background-color-error"),
        }
    }
}
