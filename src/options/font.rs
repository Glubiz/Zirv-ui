//! Text Styling Enums
//!
//! This module defines several enums representing different text styling properties: `FontSize`,
//! `FontWeight`, `FontStyle`, `FontFamily`, `TextAlign`, `TextTransform`, `TextDecoration`,
//! `TextOverflow`, `TextShadow`, `TextStroke`, and `TextColor`. Each enum can be converted into
//! Yew's `Classes` for CSS styling.
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
//! use zirv_ui::options::font::{
//!     FontSize,
//!     TextAlign,
//!     TextColor,
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let font_size_class: Classes = (&FontSize::Large).into();
//!     let text_align_class: Classes = (&TextAlign::Center).into();
//!     let text_color_class: Classes = (&TextColor::Primary).into();
//!
//!     html! {
//!         <div class={classes!(font_size_class, text_align_class, text_color_class)}>
//!             {"This text has large font size, centered alignment, and primary color"}
//!         </div>
//!     }
//! }
//! ```

use yew::{
    classes,
    Classes,
};

/// Enum representing the font size options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontSize {
    /// Default font size.
    #[default]
    None,
    /// Small font size.
    Small,
    /// Medium font size.
    Medium,
    /// Large font size.
    Large,
    /// Extra large font size.
    ExtraLarge,
}

impl From<&FontSize> for Classes {
    /// Converts a `FontSize` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::FontSize;
    ///
    /// let font_size_class: Classes = (&FontSize::Large).into();
    /// ```
    fn from(font_size: &FontSize) -> Self {
        match font_size {
            FontSize::None => classes!("font-size-default"),
            FontSize::Small => classes!("font-size-small"),
            FontSize::Medium => classes!("font-size-medium"),
            FontSize::Large => classes!("font-size-large"),
            FontSize::ExtraLarge => classes!("font-size-extra-large"),
        }
    }
}

/// Enum representing the font weight options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontWeight {
    /// Lighter font weight.
    Lighter,
    /// Normal font weight. This is the default.
    #[default]
    Normal,
    /// Bold font weight.
    Bold,
    /// Bolder font weight.
    Bolder,
}

impl From<&FontWeight> for Classes {
    /// Converts a `FontWeight` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::FontWeight;
    ///
    /// let font_weight_class: Classes = (&FontWeight::Bold).into();
    /// ```
    fn from(font_weight: &FontWeight) -> Self {
        match font_weight {
            FontWeight::Lighter => classes!("font-weight-lighter"),
            FontWeight::Normal => classes!("font-weight-normal"),
            FontWeight::Bold => classes!("font-weight-bold"),
            FontWeight::Bolder => classes!("font-weight-bolder"),
        }
    }
}

/// Enum representing the font style options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontStyle {
    /// Normal font style. This is the default.
    #[default]
    Normal,
    /// Italic font style.
    Italic,
}

impl From<&FontStyle> for Classes {
    /// Converts a `FontStyle` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::FontStyle;
    ///
    /// let font_style_class: Classes = (&FontStyle::Italic).into();
    /// ```
    fn from(font_style: &FontStyle) -> Self {
        match font_style {
            FontStyle::Normal => classes!("font-style-normal"),
            FontStyle::Italic => classes!("font-style-italic"),
        }
    }
}

/// Enum representing the font family options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontFamily {
    /// Arial font family. This is the default.
    #[default]
    Arial,
    /// Helvetica font family.
    Helvetica,
    /// Times New Roman font family.
    TimesNewRoman,
    /// Courier New font family.
    CourierNew,
}

impl From<&FontFamily> for Classes {
    /// Converts a `FontFamily` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::FontFamily;
    ///
    /// let font_family_class: Classes = (&FontFamily::Helvetica).into();
    /// ```
    fn from(font_family: &FontFamily) -> Self {
        match font_family {
            FontFamily::Arial => classes!("font-family-arial"),
            FontFamily::Helvetica => classes!("font-family-helvetica"),
            FontFamily::TimesNewRoman => classes!("font-family-times-new-roman"),
            FontFamily::CourierNew => classes!("font-family-courier-new"),
        }
    }
}

/// Enum representing the text align options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextAlign {
    /// Align text to the left. This is the default.
    #[default]
    Left,
    /// Align text to the center.
    Center,
    /// Align text to the right.
    Right,
    /// Justify text.
    Justify,
}

impl From<&TextAlign> for Classes {
    /// Converts a `TextAlign` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::TextAlign;
    ///
    /// let text_align_class: Classes = (&TextAlign::Center).into();
    /// ```
    fn from(text_align: &TextAlign) -> Self {
        match text_align {
            TextAlign::Left => classes!("text-align-left"),
            TextAlign::Center => classes!("text-align-center"),
            TextAlign::Right => classes!("text-align-right"),
            TextAlign::Justify => classes!("text-align-justify"),
        }
    }
}

/// Enum representing the text transform options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextTransform {
    /// No text transform. This is the default.
    #[default]
    None,
    /// Transform text to uppercase.
    Uppercase,
    /// Transform text to lowercase.
    Lowercase,
    /// Capitalize text.
    Capitalize,
}

impl From<&TextTransform> for Classes {
    /// Converts a `TextTransform` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::TextTransform;
    ///
    /// let text_transform_class: Classes = (&TextTransform::Uppercase).into();
    /// ```
    fn from(text_transform: &TextTransform) -> Self {
        match text_transform {
            TextTransform::None => classes!("text-transform-none"),
            TextTransform::Uppercase => classes!("text-transform-uppercase"),
            TextTransform::Lowercase => classes!("text-transform-lowercase"),
            TextTransform::Capitalize => classes!("text-transform-capitalize"),
        }
    }
}

/// Enum representing the text decoration options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextDecoration {
    /// No text decoration. This is the default.
    #[default]
    None,
    /// Underline text.
    Underline,
    /// Overline text.
    Overline,
    /// Line through text.
    LineThrough,
}

impl From<&TextDecoration> for Classes {
    /// Converts a `TextDecoration` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::TextDecoration;
    ///
    /// let text_decoration_class: Classes = (&TextDecoration::Underline).into();
    /// ```
    fn from(text_decoration: &TextDecoration) -> Self {
        match text_decoration {
            TextDecoration::None => classes!("text-decoration-none"),
            TextDecoration::Underline => classes!("text-decoration-underline"),
            TextDecoration::Overline => classes!("text-decoration-overline"),
            TextDecoration::LineThrough => classes!("text-decoration-line-through"),
        }
    }
}

/// Enum representing the text overflow options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextOverflow {
    /// No text overflow. This is the default.
    #[default]
    None,
    /// Wrap text overflow.
    Wrap,
    /// Ellipsis for text overflow.
    Ellipsis,
}

impl From<&TextOverflow> for Classes {
    /// Converts a `TextOverflow` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::TextOverflow;
    ///
    /// let text_overflow_class: Classes = (&TextOverflow::Ellipsis).into();
    /// ```
    fn from(text_overflow: &TextOverflow) -> Self {
        match text_overflow {
            TextOverflow::None => classes!("text-overflow-nowrap"),
            TextOverflow::Wrap => classes!("text-overflow-wrap"),
            TextOverflow::Ellipsis => classes!("text-overflow-ellipsis"),
        }
    }
}

/// Enum representing the text shadow options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextShadow {
    /// No text shadow. This is the default.
    #[default]
    None,
    /// Text shadow.
    Shadow,
}

impl From<&TextShadow> for Classes {
    /// Converts a `TextShadow` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::TextShadow;
    ///
    /// let text_shadow_class: Classes = (&TextShadow::Shadow).into();
    /// ```
    fn from(text_shadow: &TextShadow) -> Self {
        match text_shadow {
            TextShadow::None => classes!("text-shadow-none"),
            TextShadow::Shadow => classes!("text-shadow"),
        }
    }
}

/// Enum representing the text stroke options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextStroke {
    /// No text stroke. This is the default.
    #[default]
    None,
    /// Text stroke.
    Stroke,
}

impl From<&TextStroke> for Classes {
    /// Converts a `TextStroke` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::TextStroke;
    ///
    /// let text_stroke_class: Classes = (&TextStroke::Stroke).into();
    /// ```
    fn from(text_stroke: &TextStroke) -> Self {
        match text_stroke {
            TextStroke::None => classes!("text-stroke-none"),
            TextStroke::Stroke => classes!("text-stroke"),
        }
    }
}

/// Enum representing the text color options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextColor {
    /// No text color.
    None,
    /// Primary text color.
    Primary,
    /// Secondary text color.
    Secondary,
    /// Tertiary text color.
    Tertiary,
    /// Container text color.
    Container,
    /// Background text color.
    Background,
    /// Primary text color (default).
    #[default]
    TextPrimary,
    /// Secondary text color.
    TextSecondary,
    /// Success text color.
    Success,
    /// Warning text color.
    Warning,
    /// Error text color.
    Error,
}

impl From<&TextColor> for Classes {
    /// Converts a `TextColor` into Yew's `Classes`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yew::Classes;
    /// use zirv_ui::options::font::TextColor;
    ///
    /// let text_color_class: Classes = (&TextColor::Primary).into();
    /// ```
    fn from(text_color: &TextColor) -> Self {
        match text_color {
            TextColor::None => classes!("text-color-none"),
            TextColor::Primary => classes!("text-color-primary"),
            TextColor::Secondary => classes!("text-color-secondary"),
            TextColor::Tertiary => classes!("text-color-tertiary"),
            TextColor::Container => classes!("text-color-container"),
            TextColor::Background => classes!("text-color-background"),
            TextColor::TextPrimary => classes!("text-color-text-primary"),
            TextColor::TextSecondary => classes!("text-color-text-secondary"),
            TextColor::Success => classes!("text-color-success"),
            TextColor::Warning => classes!("text-color-warning"),
            TextColor::Error => classes!("text-color-error"),
        }
    }
}
