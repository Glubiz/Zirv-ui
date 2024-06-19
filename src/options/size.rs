//! Size Enums
//! 
//! This module defines several enums representing different size properties: `Height`, `Width`, `MinHeight`,
//! `MaxHeight`, `MinWidth`, and `MaxWidth`. Each enum can be converted into Yew's `Classes` for CSS styling.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html, function_component, Html};
//! use crate::options::size::{Height, Width};
//! use yew::Classes;
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     let height_class: Classes = (&Height::Large).into();
//!     let width_class: Classes = (&Width::Full).into();
//! 
//!     html! {
//!         <div class={classes!(height_class, width_class)}>
//!             {"This div has large height and full width"}
//!         </div>
//!     }
//! }
//! ```

use yew::{classes, Classes};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CustomType {
    #[default]
    Fixed,
    Percent
}

/// Enum representing the height property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Height {
    /// No height.
    None,
    /// Small height.
    #[default]
    Small,
    /// Medium height.
    Medium,
    /// Large height.
    Large,
    /// Full height.
    Full,
    /// Screen height.
    Screen,
    /// Auto height.
    Auto,
    /// Minimum height.
    Min,
    /// Maximum height.
    Max,
    /// Fit height.
    Fit,
    /// Inherit height.
    Inherit,
    /// Custom height.
    Custom(u8, CustomType),
}

impl From<&Height> for Classes {
    /// Converts a `Height` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::size::Height;
    /// 
    /// let height_class: Classes = (&Height::Large).into();
    /// ```
    fn from(height: &Height) -> Self {
        match height {
            Height::None => classes!("height-none"),
            Height::Small => classes!("height-small"),
            Height::Medium => classes!("height-medium"),
            Height::Large => classes!("height-large"),
            Height::Full => classes!("height-full"),
            Height::Screen => classes!("height-screen"),
            Height::Auto => classes!("height-auto"),
            Height::Min => classes!("height-min"),
            Height::Max => classes!("height-max"),
            Height::Fit => classes!("height-fit"),
            Height::Inherit => classes!("height-inherit"),
            Height::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("height-fixed-{}", value)),
                CustomType::Percent => classes!(format!("height-percent-{}", value)),
            }
        }
    }
}

/// Enum representing the width property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Width {
    /// No width.
    None,
    /// Small width.
    #[default]
    Small,
    /// Medium width.
    Medium,
    /// Large width.
    Large,
    /// Full width.
    Full,
    /// Screen width.
    Screen,
    /// Auto width.
    Auto,
    /// Minimum width.
    Min,
    /// Maximum width.
    Max,
    /// Fit width.
    Fit,
    /// Inherit width.
    Inherit,
    /// Custom width.
    Custom(u8, CustomType),
}

impl From<&Width> for Classes {
    /// Converts a `Width` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::size::Width;
    /// 
    /// let width_class: Classes = (&Width::Full).into();
    /// ```
    fn from(width: &Width) -> Self {
        match width {
            Width::None => classes!("width-none"),
            Width::Small => classes!("width-small"),
            Width::Medium => classes!("width-medium"),
            Width::Large => classes!("width-large"),
            Width::Full => classes!("width-full"),
            Width::Screen => classes!("width-screen"),
            Width::Auto => classes!("width-auto"),
            Width::Min => classes!("width-min"),
            Width::Max => classes!("width-max"),
            Width::Fit => classes!("width-fit"),
            Width::Inherit => classes!("width-inherit"),
            Width::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("width-fixed-{}", value)),
                CustomType::Percent => classes!(format!("width-percent-{}", value)),   
            }
        }
    }
}

/// Enum representing the minimum height property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MinHeight {
    /// No minimum height.
    None,
    /// Small minimum height.
    #[default]
    Small,
    /// Medium minimum height.
    Medium,
    /// Large minimum height.
    Large,
    /// Full minimum height.
    Full,
    /// Screen minimum height.
    Screen,
    /// Auto minimum height.
    Auto,
    /// Minimum height.
    Min,
    /// Maximum height.
    Max,
    /// Fit minimum height.
    Fit,
    /// Inherit minimum height.
    Inherit,
    /// Custom minimum height.
    Custom(u8, CustomType),
}

impl From<&MinHeight> for Classes {
    /// Converts a `MinHeight` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::size::MinHeight;
    /// 
    /// let min_height_class: Classes = (&MinHeight::Large).into();
    /// ```
    fn from(min_height: &MinHeight) -> Self {
        match min_height {
            MinHeight::None => classes!("min-height-none"),
            MinHeight::Small => classes!("min-height-small"),
            MinHeight::Medium => classes!("min-height-medium"),
            MinHeight::Large => classes!("min-height-large"),
            MinHeight::Full => classes!("min-height-full"),
            MinHeight::Screen => classes!("min-height-screen"),
            MinHeight::Auto => classes!("min-height-auto"),
            MinHeight::Min => classes!("min-height-min"),
            MinHeight::Max => classes!("min-height-max"),
            MinHeight::Fit => classes!("min-height-fit"),
            MinHeight::Inherit => classes!("min-height-inherit"),
            MinHeight::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("min-height-fixed-{}", value)),
                CustomType::Percent => classes!(format!("min-height-percent-{}", value)),   
            },
        }
    }
}

/// Enum representing the maximum height property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MaxHeight {
    /// No maximum height.
    None,
    /// Small maximum height.
    #[default]
    Small,
    /// Medium maximum height.
    Medium,
    /// Large maximum height.
    Large,
    /// Full maximum height.
    Full,
    /// Screen maximum height.
    Screen,
    /// Auto maximum height.
    Auto,
    /// Minimum maximum height.
    Min,
    /// Maximum maximum height.
    Max,
    /// Fit maximum height.
    Fit,
    /// Inherit maximum height.
    Inherit,
    /// Custom maximum height.
    Custom(u8, CustomType),
}

impl From<&MaxHeight> for Classes {
    /// Converts a `MaxHeight` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::size::MaxHeight;
    /// 
    /// let max_height_class: Classes = (&MaxHeight::Large).into();
    /// ```
    fn from(max_height: &MaxHeight) -> Self {
        match max_height {
            MaxHeight::None => classes!("max-height-none"),
            MaxHeight::Small => classes!("max-height-small"),
            MaxHeight::Medium => classes!("max-height-medium"),
            MaxHeight::Large => classes!("max-height-large"),
            MaxHeight::Full => classes!("max-height-full"),
            MaxHeight::Screen => classes!("max-height-screen"),
            MaxHeight::Auto => classes!("max-height-auto"),
            MaxHeight::Min => classes!("max-height-min"),
            MaxHeight::Max => classes!("max-height-max"),
            MaxHeight::Fit => classes!("max-height-fit"),
            MaxHeight::Inherit => classes!("max-height-inherit"),
            MaxHeight::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("max-height-fixed-{}", value)),
                CustomType::Percent => classes!(format!("max-height-percent-{}", value)),   
            },
        }
    }
}

/// Enum representing the minimum width property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MinWidth {
    /// No minimum width.
    None,
    /// Small minimum width.
    #[default]
    Small,
    /// Medium minimum width.
    Medium,
    /// Large minimum width.
    Large,
    /// Full minimum width.
    Full,
    /// Screen minimum width.
    Screen,
    /// Auto minimum width.
    Auto,
    /// Minimum width.
    Min,
    /// Maximum width.
    Max,
    /// Fit minimum width.
    Fit,
    /// Inherit minimum width.
    Inherit,
    /// Custom minimum width.
    Custom(u8, CustomType),
}

impl From<&MinWidth> for Classes {
    /// Converts a `MinWidth` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::size::MinWidth;
    /// 
    /// let min_width_class: Classes = (&MinWidth::Large).into();
    /// ```
    fn from(min_width: &MinWidth) -> Self {
        match min_width {
            MinWidth::None => classes!("min-width-none"),
            MinWidth::Small => classes!("min-width-small"),
            MinWidth::Medium => classes!("min-width-medium"),
            MinWidth::Large => classes!("min-width-large"),
            MinWidth::Full => classes!("min-width-full"),
            MinWidth::Screen => classes!("min-width-screen"),
            MinWidth::Auto => classes!("min-width-auto"),
            MinWidth::Min => classes!("min-width-min"),
            MinWidth::Max => classes!("min-width-max"),
            MinWidth::Fit => classes!("min-width-fit"),
            MinWidth::Inherit => classes!("min-width-inherit"),
            MinWidth::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("min-width-fixed-{}", value)),
                CustomType::Percent => classes!(format!("min-width-percent-{}", value)),   
            },
        }
    }
}

/// Enum representing the maximum width property options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum MaxWidth {
    /// No maximum width.
    None,
    /// Small maximum width.
    #[default]
    Small,
    /// Medium maximum width.
    Medium,
    /// Large maximum width.
    Large,
    /// Full maximum width.
    Full,
    /// Screen maximum width.
    Screen,
    /// Auto maximum width.
    Auto,
    /// Minimum maximum width.
    Min,
    /// Maximum maximum width.
    Max,
    /// Fit maximum width.
    Fit,
    /// Inherit maximum width.
    Inherit,
    /// Custom maximum width.
    Custom(u8, CustomType),
}

impl From<&MaxWidth> for Classes {
    /// Converts a `MaxWidth` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::size::MaxWidth;
    /// 
    /// let max_width_class: Classes = (&MaxWidth::Large).into();
    /// ```
    fn from(max_width: &MaxWidth) -> Self {
        match max_width {
            MaxWidth::None => classes!("max-width-none"),
            MaxWidth::Small => classes!("max-width-small"),
            MaxWidth::Medium => classes!("max-width-medium"),
            MaxWidth::Large => classes!("max-width-large"),
            MaxWidth::Full => classes!("max-width-full"),
            MaxWidth::Screen => classes!("max-width-screen"),
            MaxWidth::Auto => classes!("max-width-auto"),
            MaxWidth::Min => classes!("max-width-min"),
            MaxWidth::Max => classes!("max-width-max"),
            MaxWidth::Fit => classes!("max-width-fit"),
            MaxWidth::Inherit => classes!("max-width-inherit"),
            MaxWidth::Custom(value, _type) => match _type {
                CustomType::Fixed => classes!(format!("max-width-fixed-{}", value)),
                CustomType::Percent => classes!(format!("max-width-percent-{}", value)),   
            },
        }
    }
}