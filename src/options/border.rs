use yew::{classes, Classes};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BorderRaduis {
    None,
    Small,
    #[default]
    Medium,
    Large,
    Rounded,
}

impl From<&BorderRaduis> for Classes {
    fn from(border_radius: &BorderRaduis) -> Self {
        match border_radius {
            BorderRaduis::None => classes!("border-radius-none"),
            BorderRaduis::Small => classes!("border-radius-small"),
            BorderRaduis::Medium => classes!("border-radius-medium"),
            BorderRaduis::Large => classes!("border-radius-large"),
            BorderRaduis::Rounded => classes!("border-radius-rounded"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BorderWidth {
    #[default]
    None,
    Small,
    Medium,
    Large,
}

impl From<&BorderWidth> for Classes {
    fn from(border_width: &BorderWidth) -> Self {
        match border_width {
            BorderWidth::None => classes!("border-width-none"),
            BorderWidth::Small => classes!("border-width-small"),
            BorderWidth::Medium => classes!("border-width-medium"),
            BorderWidth::Large => classes!("border-width-large"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
}

impl From<&BorderStyle> for Classes {
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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BorderColor {
    #[default]
    None,
    Primary,
    Secondary,
    Tertiary,
    Success,
    Warning,
    Error,
}

impl From<&BorderColor> for Classes {
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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Border {
    #[default]
    None,
    All(BorderWidth, BorderStyle, BorderColor),
    Top(BorderWidth, BorderStyle, BorderColor),
    Right(BorderWidth, BorderStyle, BorderColor),
    Bottom(BorderWidth, BorderStyle, BorderColor),
    Left(BorderWidth, BorderStyle, BorderColor),
}

impl From<&Border> for Classes {
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
                let mut classes =classes!("border-bottom");
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
