//! Spacing Enums
//!
//! This module defines the `Padding` and `Margin` enums, which represent various spacing property
//! options for CSS. It also provides an implementation to convert these enums into Yew's `Classes`
//! for CSS styling.
//!
//! # Example
//!
//! ```rust
//! use yew::{
//!     classes,
//!     function_component,
//!     html,
//!     Classes,
//!     Html,
//! };
//! use zirv_ui::options::spacing::{
//!     Margin,
//!     Padding,
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <div class={classes!(&Margin::Medium, &Padding::Medium)}>
//!             {"This div has large padding and medium margin"}
//!         </div>
//!     }
//! }
//! ```

use yew::{
    classes,
    Classes,
};

use crate::size::CustomType;

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
    Custom(u8, CustomType),
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
            Margin::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("margin-fixed-{}", value)),
                CustomType::Percent => classes!(format!("margin-percent-{}", value)),
            },
        }
    }
}

/// Enum representing the margin property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MarginLeft {
    /// No margin.
    None,
    /// Small margin. This is the default.
    #[default]
    Small,
    /// Medium margin.
    Medium,
    /// Large margin.
    Large,
    Custom(u8, CustomType),
}

impl From<&MarginLeft> for Classes {
    /// Converts a `Margin` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::spacing::MarginLeft;
    ///
    /// let margin_class: Classes = (&MarginLeft::Medium).into();
    /// ```
    fn from(margin: &MarginLeft) -> Self {
        match margin {
            MarginLeft::None => classes!("margin-left-none"),
            MarginLeft::Small => classes!("margin-left-small"),
            MarginLeft::Medium => classes!("margin-left-medium"),
            MarginLeft::Large => classes!("margin-left-large"),
            MarginLeft::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("margin-left-fixed-{}", value)),
                CustomType::Percent => classes!(format!("margin-left-percent-{}", value)),
            },
        }
    }
}

/// Enum representing the margin property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MarginRight {
    /// No margin.
    None,
    /// Small margin. This is the default.
    #[default]
    Small,
    /// Medium margin.
    Medium,
    /// Large margin.
    Large,
    Custom(u8, CustomType),
}

impl From<&MarginRight> for Classes {
    /// Converts a `Margin` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::spacing::MarginRight;
    ///
    /// let margin_class: Classes = (&MarginRight::Medium).into();
    /// ```
    fn from(margin: &MarginRight) -> Self {
        match margin {
            MarginRight::None => classes!("margin-right-none"),
            MarginRight::Small => classes!("margin-right-small"),
            MarginRight::Medium => classes!("margin-right-medium"),
            MarginRight::Large => classes!("margin-right-large"),
            MarginRight::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("margin-right-fixed-{}", value)),
                CustomType::Percent => classes!(format!("margin-right-percent-{}", value)),
            },
        }
    }
}

/// Enum representing the margin property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MarginTop {
    /// No margin.
    None,
    /// Small margin. This is the default.
    #[default]
    Small,
    /// Medium margin.
    Medium,
    /// Large margin.
    Large,
    Custom(u8, CustomType),
}

impl From<&MarginTop> for Classes {
    /// Converts a `Margin` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::spacing::MarginTop;
    ///
    /// let margin_class: Classes = (&MarginTop::Medium).into();
    /// ```
    fn from(margin: &MarginTop) -> Self {
        match margin {
            MarginTop::None => classes!("margin-top-none"),
            MarginTop::Small => classes!("margin-top-small"),
            MarginTop::Medium => classes!("margin-top-medium"),
            MarginTop::Large => classes!("margin-top-large"),
            MarginTop::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("margin-top-fixed-{}", value)),
                CustomType::Percent => classes!(format!("margin-top-percent-{}", value)),
            },
        }
    }
}

/// Enum representing the margin property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MarginBottom {
    /// No margin.
    None,
    /// Small margin. This is the default.
    #[default]
    Small,
    /// Medium margin.
    Medium,
    /// Large margin.
    Large,
    Custom(u8, CustomType),
}

impl From<&MarginBottom> for Classes {
    /// Converts a `Margin` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::spacing::MarginBottom;
    ///
    /// let margin_class: Classes = (&MarginBottom::Medium).into();
    /// ```
    fn from(margin: &MarginBottom) -> Self {
        match margin {
            MarginBottom::None => classes!("margin-bottom-none"),
            MarginBottom::Small => classes!("margin-bottom-small"),
            MarginBottom::Medium => classes!("margin-bottom-medium"),
            MarginBottom::Large => classes!("margin-bottom-large"),
            MarginBottom::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("margin-bottom-fixed-{}", value)),
                CustomType::Percent => classes!(format!("margin-bottom-percent-{}", value)),
            },
        }
    }
}
