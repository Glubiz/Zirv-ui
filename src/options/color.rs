use yew::{classes, Classes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BackgroundColor {
    None,
    Primary,
    Secondary,
    Tertiary,
    Container,
    #[default]
    Background,
    TextPrimary,
    TextSecondary,
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
            BackgroundColor::Container => classes!("background-color-container"),
            BackgroundColor::Background => classes!("background-color-background"),
            BackgroundColor::TextPrimary => classes!("background-color-text-primary"),
            BackgroundColor::TextSecondary => classes!("background-color-text-secondary"),
            BackgroundColor::Success => classes!("background-color-success"),
            BackgroundColor::Warning => classes!("background-color-warning"),
            BackgroundColor::Error => classes!("background-color-error"),
        }
    }
}