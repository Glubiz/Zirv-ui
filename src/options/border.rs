//! Border and BorderStyle Enums
//!
//! This module defines several enums representing different border properties: `BorderRadius`, `BorderWidth`,
//! `BorderStyle`, `BorderColor`, and `Border`. Each enum can be converted into Yew's `Classes` for CSS styling.
//!
//! # Example
//!
//! ```rust
//! use yew::{html, function_component, Html, Classes};
//! use zirv_ui::options::border::{Border, BorderColor, BorderRadius, BorderStyle, BorderWidth};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let border_class: Classes = (&Border::All(BorderWidth::Medium, BorderStyle::Solid, BorderColor::Primary)).into();
//!
//!     html! {
//!         <div class={border_class}>
//!             {"This div has a medium solid primary border"}
//!         </div>
//!     }
//! }
//! ```

use yew::{classes, Classes};

/// Enum representing the border radius options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum BorderRadius {
    /// No border radius.
    None,
    /// Small border radius.
    Small,
    /// Medium border radius. This is the default.
    #[default]
    Medium,
    /// Large border radius.
    Large,
    /// Fully rounded border radius.
    Rounded,
}

impl From<&BorderRadius> for Classes {
    /// Converts a `BorderRadius` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::border::BorderRadius;
    ///
    /// let radius_class: Classes = (&BorderRadius::Medium).into();
    /// ```
    fn from(border_radius: &BorderRadius) -> Self {
        match border_radius {
            BorderRadius::None => classes!("border-radius-none"),
            BorderRadius::Small => classes!("border-radius-small"),
            BorderRadius::Medium => classes!("border-radius-medium"),
            BorderRadius::Large => classes!("border-radius-large"),
            BorderRadius::Rounded => classes!("border-radius-rounded"),
        }
    }
}

/// Enum representing the border width options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum BorderWidth {
    /// No border width. This is the default.
    #[default]
    None,
    /// Small border width.
    Small,
    /// Medium border width.
    Medium,
    /// Large border width.
    Large,
}

impl From<&BorderWidth> for Classes {
    /// Converts a `BorderWidth` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::border::BorderWidth;
    ///
    /// let width_class: Classes = (&BorderWidth::Medium).into();
    /// ```
    fn from(border_width: &BorderWidth) -> Self {
        match border_width {
            BorderWidth::None => classes!("border-width-none"),
            BorderWidth::Small => classes!("border-width-small"),
            BorderWidth::Medium => classes!("border-width-medium"),
            BorderWidth::Large => classes!("border-width-large"),
        }
    }
}

/// Enum representing the border style options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum BorderStyle {
    /// No border style. This is the default.
    #[default]
    None,
    /// Solid border style.
    Solid,
    /// Dashed border style.
    Dashed,
    /// Dotted border style.
    Dotted,
    /// Double border style.
    Double,
}

impl From<&BorderStyle> for Classes {
    /// Converts a `BorderStyle` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::border::BorderStyle;
    ///
    /// let style_class: Classes = (&BorderStyle::Solid).into();
    /// ```
    fn from(border_style: &BorderStyle) -> Self {
        match border_style {
            BorderStyle::None => classes!("border-style-none"),
            BorderStyle::Solid => classes!("border-style-solid"),
            BorderStyle::Dashed => classes!("border-style-dashed"),
            BorderStyle::Dotted => classes!("border-style-dotted"),
            BorderStyle::Double => classes!("border-style-double"),
        }
    }
}

/// Enum representing the border color options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum BorderColor {
    /// No border color. This is the default.
    #[default]
    None,
    /// Primary border color.
    Primary,
    /// Secondary border color.
    Secondary,
    /// Tertiary border color.
    Tertiary,
    /// Success border color.
    Success,
    /// Warning border color.
    Warning,
    /// Error border color.
    Error,
}

impl From<&BorderColor> for Classes {
    /// Converts a `BorderColor` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::border::BorderColor;
    ///
    /// let color_class: Classes = (&BorderColor::Primary).into();
    /// ```
    fn from(border_color: &BorderColor) -> Self {
        match border_color {
            BorderColor::None => classes!("border-color-none"),
            BorderColor::Primary => classes!("border-color-primary"),
            BorderColor::Secondary => classes!("border-color-secondary"),
            BorderColor::Tertiary => classes!("border-color-tertiary"),
            BorderColor::Success => classes!("border-color-success"),
            BorderColor::Warning => classes!("border-color-warning"),
            BorderColor::Error => classes!("border-color-error"),
        }
    }
}

/// Enum representing the border options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Border {
    /// No border. This is the default.
    #[default]
    None,
    /// Border applied to all sides with specified width, style, and color.
    All(BorderWidth, BorderStyle, BorderColor),
    /// Border applied to the top side with specified width, style, and color.
    Top(BorderWidth, BorderStyle, BorderColor),
    /// Border applied to the right side with specified width, style, and color.
    Right(BorderWidth, BorderStyle, BorderColor),
    /// Border applied to the bottom side with specified width, style, and color.
    Bottom(BorderWidth, BorderStyle, BorderColor),
    /// Border applied to the left side with specified width, style, and color.
    Left(BorderWidth, BorderStyle, BorderColor),
}

impl From<&Border> for Classes {
    /// Converts a `Border` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::border::{Border, BorderWidth, BorderStyle, BorderColor};
    ///
    /// let border_class: Classes = (&Border::All(BorderWidth::Medium, BorderStyle::Solid, BorderColor::Primary)).into();
    /// ```
    fn from(border: &Border) -> Self {
        match border {
            Border::None => classes!("border-none"),
            Border::All(width, style, color) => {
                let mut classes = classes!("border-all");
                classes.push(width);
                classes.push(style);
                classes.push(color);
                classes
            }
            Border::Top(width, style, color) => {
                let mut classes = classes!("border-top");
                classes.push(width);
                classes.push(style);
                classes.push(color);
                classes
            }
            Border::Right(width, style, color) => {
                let mut classes = classes!("border-right");
                classes.push(width);
                classes.push(style);
                classes.push(color);
                classes
            }
            Border::Bottom(width, style, color) => {
                let mut classes = classes!("border-bottom");
                classes.push(width);
                classes.push(style);
                classes.push(color);
                classes
            }
            Border::Left(width, style, color) => {
                let mut classes = classes!("border-left");
                classes.push(width);
                classes.push(style);
                classes.push(color);
                classes
            }
        }
    }
}
