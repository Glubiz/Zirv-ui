use yew::{classes, Classes};

pub enum BackgroundColor {
    None,
    Primary,
    Secondary,
    Tertiary,
    Success,
    Warning,
    Error,
}

impl From<&BackgroundColor> for Classes {
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