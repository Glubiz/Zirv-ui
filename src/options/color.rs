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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BackgroundColor {
    /// No background color.
    None,
    /// Primary background color.
    Primary,
    /// Dark variant of the primary background color.
    PrimaryDark,
    /// Light variant of the primary background color.
    PrimaryLight,
    /// Secondary background color.
    Secondary,
    /// Dark variant of the secondary background color.
    SecondaryDark,
    /// Light variant of the secondary background color.
    SecondaryLight,
    /// Tertiary background color.
    Tertiary,
    /// Dark variant of the tertiary background color.
    TertiaryDark,
    /// Light variant of the tertiary background color.
    TertiaryLight,
    /// Container background color.
    Container,
    /// Dark variant of the container background color.
    ContainerDark,
    /// Light variant of the container background color.
    ContainerLight,
    /// Background color.
    #[default]
    Background,
    /// Dark variant of the background color.
    BackgroundDark,
    /// Light variant of the background color.
    BackgroundLight,
    /// Primary text background color.
    TextPrimary,
    /// Dark variant of the primary text background color.
    TextPrimaryDark,
    /// Light variant of the primary text background color.
    TextPrimaryLight,
    /// Secondary text background color.
    TextSecondary,
    /// Dark variant of the secondary text background color.
    TextSecondaryDark,
    /// Light variant of the secondary text background color.
    TextSecondaryLight,
    /// Success background color.
    Success,
    /// Dark variant of the success background color.
    SuccessDark,
    /// Light variant of the success background color.
    SuccessLight,
    /// Warning background color.
    Warning,
    /// Dark variant of the warning background color.
    WarningDark,
    /// Light variant of the warning background color.
    WarningLight,
    /// Error background color.
    Error,
    /// Dark variant of the error background color.
    ErrorDark,
    /// Light variant of the error background color.
    ErrorLight,
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
            BackgroundColor::PrimaryDark => classes!("background-color-primary-dark"),
            BackgroundColor::PrimaryLight => classes!("background-color-primary-light"),
            BackgroundColor::Secondary => classes!("background-color-secondary"),
            BackgroundColor::SecondaryDark => classes!("background-color-secondary-dark"),
            BackgroundColor::SecondaryLight => classes!("background-color-secondary-light"),
            BackgroundColor::Tertiary => classes!("background-color-tertiary"),
            BackgroundColor::TertiaryDark => classes!("background-color-tertiary-dark"),
            BackgroundColor::TertiaryLight => classes!("background-color-tertiary-light"),
            BackgroundColor::Container => classes!("background-color-container"),
            BackgroundColor::ContainerDark => classes!("background-color-container-dark"),
            BackgroundColor::ContainerLight => classes!("background-color-container-light"),
            BackgroundColor::Background => classes!("background-color-background"),
            BackgroundColor::BackgroundDark => classes!("background-color-background-dark"),
            BackgroundColor::BackgroundLight => classes!("background-color-background-light"),
            BackgroundColor::TextPrimary => classes!("background-color-text-primary"),
            BackgroundColor::TextPrimaryDark => classes!("background-color-text-primary-dark"),
            BackgroundColor::TextPrimaryLight => classes!("background-color-text-primary-light"),
            BackgroundColor::TextSecondary => classes!("background-color-text-secondary"),
            BackgroundColor::TextSecondaryDark => classes!("background-color-text-secondary-dark"),
            BackgroundColor::TextSecondaryLight => classes!("background-color-text-secondary-light"),
            BackgroundColor::Success => classes!("background-color-success"),
            BackgroundColor::SuccessDark => classes!("background-color-success-dark"),
            BackgroundColor::SuccessLight => classes!("background-color-success-light"),
            BackgroundColor::Warning => classes!("background-color-warning"),
            BackgroundColor::WarningDark => classes!("background-color-warning-dark"),
            BackgroundColor::WarningLight => classes!("background-color-warning-light"),
            BackgroundColor::Error => classes!("background-color-error"),
            BackgroundColor::ErrorDark => classes!("background-color-error-dark"),
            BackgroundColor::ErrorLight => classes!("background-color-error-light"),
        }
    }
}

impl BackgroundColor {
    pub fn to_dark(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Primary | Self::PrimaryDark | Self::PrimaryLight => Self::PrimaryDark,
            Self::Secondary | Self::SecondaryDark | Self::SecondaryLight => Self::SecondaryDark,
            Self::Tertiary | Self::TertiaryDark | Self::TertiaryLight => Self::TertiaryDark,
            Self::Container | Self::ContainerDark | Self::ContainerLight => Self::ContainerDark,
            Self::Background | Self::BackgroundDark | Self::BackgroundLight => Self::BackgroundDark,
            Self::TextPrimary | Self::TextPrimaryDark | Self::TextPrimaryLight => Self::TextPrimaryDark,
            Self::TextSecondary | Self::TextSecondaryDark | Self::TextSecondaryLight => Self::TextSecondaryDark,
            Self::Success | Self::SuccessDark | Self::SuccessLight => Self::SuccessDark,
            Self::Warning | Self::WarningDark | Self::WarningLight => Self::WarningDark,
            Self::Error | Self::ErrorDark | Self::ErrorLight => Self::ErrorDark,
        }
    }

    pub fn to_light(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Primary | Self::PrimaryDark | Self::PrimaryLight => Self::PrimaryLight,
            Self::Secondary | Self::SecondaryDark | Self::SecondaryLight => Self::SecondaryLight,
            Self::Tertiary | Self::TertiaryDark | Self::TertiaryLight => Self::TertiaryLight,
            Self::Container | Self::ContainerDark | Self::ContainerLight => Self::ContainerLight,
            Self::Background | Self::BackgroundDark | Self::BackgroundLight => Self::BackgroundLight,
            Self::TextPrimary | Self::TextPrimaryDark | Self::TextPrimaryLight => Self::TextPrimaryLight,
            Self::TextSecondary | Self::TextSecondaryDark | Self::TextSecondaryLight => Self::TextSecondaryLight,
            Self::Success | Self::SuccessDark | Self::SuccessLight => Self::SuccessLight,
            Self::Warning | Self::WarningDark | Self::WarningLight => Self::WarningLight,
            Self::Error | Self::ErrorDark | Self::ErrorLight => Self::ErrorLight,
        }
    }

    pub fn to_classes(&self) -> Classes {
        match self {
            Self::None => classes!("background-color-none"),
            Self::Primary => classes!("background-color-primary"),
            Self::PrimaryDark => classes!("background-color-primary-dark"),
            Self::PrimaryLight => classes!("background-color-primary-light"),
            Self::Secondary => classes!("background-color-secondary"),
            Self::SecondaryDark => classes!("background-color-secondary-dark"),
            Self::SecondaryLight => classes!("background-color-secondary-light"),
            Self::Tertiary => classes!("background-color-tertiary"),
            Self::TertiaryDark => classes!("background-color-tertiary-dark"),
            Self::TertiaryLight => classes!("background-color-tertiary-light"),
            Self::Container => classes!("background-color-container"),
            Self::ContainerDark => classes!("background-color-container-dark"),
            Self::ContainerLight => classes!("background-color-container-light"),
            Self::Background => classes!("background-color-background"),
            Self::BackgroundDark => classes!("background-color-background-dark"),
            Self::BackgroundLight => classes!("background-color-background-light"),
            Self::TextPrimary => classes!("background-color-text-primary"),
            Self::TextPrimaryDark => classes!("background-color-text-primary-dark"),
            Self::TextPrimaryLight => classes!("background-color-text-primary-light"),
            Self::TextSecondary => classes!("background-color-text-secondary"),
            Self::TextSecondaryDark => classes!("background-color-text-secondary-dark"),
            Self::TextSecondaryLight => classes!("background-color-text-secondary-light"),
            Self::Success => classes!("background-color-success"),
            Self::SuccessDark => classes!("background-color-success-dark"),
            Self::SuccessLight => classes!("background-color-success-light"),
            Self::Warning => classes!("background-color-warning"),
            Self::WarningDark => classes!("background-color-warning-dark"),
            Self::WarningLight => classes!("background-color-warning-light"),
            Self::Error => classes!("background-color-error"),
            Self::ErrorDark => classes!("background-color-error-dark"),
            Self::ErrorLight => classes!("background-color-error-light"),
        }
    }
}
