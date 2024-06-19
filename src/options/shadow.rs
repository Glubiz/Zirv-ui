//! BoxShadow Enum
//!
//! This module defines the `BoxShadow` enum, which represents various box shadow property options for CSS.
//! It also provides an implementation to convert `BoxShadow` into Yew's `Classes` for CSS styling.
//!
//! # Example
//!
//! ```rust
//! use yew::{html, function_component, Html};
//! use zirv_ui::options::shadow::BoxShadow;
//! use yew::Classes;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let box_shadow_class: Classes = (&BoxShadow::Some).into();
//!
//!     html! {
//!         <div class={box_shadow_class}>
//!             {"This div has a box shadow"}
//!         </div>
//!     }
//! }
//! ```

use yew::{classes, Classes};

/// Enum representing the box shadow property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum BoxShadow {
    /// No box shadow. This is the default.
    #[default]
    None,
    /// Box shadow.
    Some,
}

impl From<&BoxShadow> for Classes {
    /// Converts a `BoxShadow` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::shadow::BoxShadow;
    ///
    /// let box_shadow_class: Classes = (&BoxShadow::Some).into();
    /// ```
    fn from(box_shadow: &BoxShadow) -> Self {
        match box_shadow {
            BoxShadow::None => classes!("box-shadow-none"),
            BoxShadow::Some => classes!("box-shadow"),
        }
    }
}
