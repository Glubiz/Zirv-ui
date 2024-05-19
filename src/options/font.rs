use yew::{classes, Classes};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontSize {
    #[default]
    None,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl From<&FontSize> for Classes {
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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontWeight {
    Lighter,
    #[default]
    Normal,
    Bold,
    Bolder,
}

impl From<&FontWeight> for Classes {
    fn from(font_weight: &FontWeight) -> Self {
        match font_weight {
            FontWeight::Lighter => classes!("font-weight-lighter"),
            FontWeight::Normal => classes!("font-weight-normal"),
            FontWeight::Bold => classes!("font-weight-bold"),
            FontWeight::Bolder => classes!("font-weight-bolder"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

impl From<&FontStyle> for Classes {
    fn from(font_style: &FontStyle) -> Self {
        match font_style {
            FontStyle::Normal => classes!("font-style-normal"),
            FontStyle::Italic => classes!("font-style-italic"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FontFamily {
    #[default]
    Arial,
    Helvetica,
    TimesNewRoman,
    CourierNew,
}

impl From<&FontFamily> for Classes {
    fn from(font_family: &FontFamily) -> Self {
        match font_family {
            FontFamily::Arial => classes!("font-family-arial"),
            FontFamily::Helvetica => classes!("font-family-helvetica"),
            FontFamily::TimesNewRoman => classes!("font-family-times-new-roman"),
            FontFamily::CourierNew => classes!("font-family-courier-new"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

impl From<&TextAlign> for Classes {
    fn from(text_align: &TextAlign) -> Self {
        match text_align {
            TextAlign::Left => classes!("text-align-left"),
            TextAlign::Center => classes!("text-align-center"),
            TextAlign::Right => classes!("text-align-right"),
            TextAlign::Justify => classes!("text-align-justify"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextTransform {
    #[default]
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

impl From<&TextTransform> for Classes {
    fn from(text_transform: &TextTransform) -> Self {
        match text_transform {
            TextTransform::None => classes!("text-transform-none"),
            TextTransform::Uppercase => classes!("text-transform-uppercase"),
            TextTransform::Lowercase => classes!("text-transform-lowercase"),
            TextTransform::Capitalize => classes!("text-transform-capitalize"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextDecoration {
    #[default]
    None,
    Underline,
    Overline,
    LineThrough,
}

impl From<&TextDecoration> for Classes {
    fn from(text_decoration: &TextDecoration) -> Self {
        match text_decoration {
            TextDecoration::None => classes!("text-decoration-none"),
            TextDecoration::Underline => classes!("text-decoration-underline"),
            TextDecoration::Overline => classes!("text-decoration-overline"),
            TextDecoration::LineThrough => classes!("text-decoration-line-through"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextOverflow {
    #[default]
    None,
    Wrap,
    Ellipsis,
}

impl From<&TextOverflow> for Classes {
    fn from(text_overflow: &TextOverflow) -> Self {
        match text_overflow {
            TextOverflow::None => classes!("text-overflow-nowrap"),
            TextOverflow::Wrap => classes!("text-overflow-wrap"),
            TextOverflow::Ellipsis => classes!("text-overflow-ellipsis"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextShadow {
    #[default]
    None,
    Shadow,
}

impl From<&TextShadow> for Classes {
    fn from(text_shadow: &TextShadow) -> Self {
        match text_shadow {
            TextShadow::None => classes!("text-shadow-none"),
            TextShadow::Shadow => classes!("text-shadow"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextStroke {
    #[default]
    None,
    Stroke,
}

impl From<&TextStroke> for Classes {
    fn from(text_stroke: &TextStroke) -> Self {
        match text_stroke {
            TextStroke::None => classes!("text-stroke-none"),
            TextStroke::Stroke => classes!("text-stroke"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TextColor {
    None,
    Primary,
    Secondary,
    Tertiary,
    Container,
    Background,
    #[default]
    TextPrimary,
    TextSecondary,
    Success,
    Warning,
    Error,
}

impl From<&TextColor> for Classes {
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