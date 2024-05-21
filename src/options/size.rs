use yew::{classes, Classes};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Height {
    None,
    #[default]
    Small,
    Medium,
    Large,
    Full,
    Screen,
    Auto,
    Min,
    Max,
    Fit,
    Inherit,
    Custom(u8)
}

impl From<&Height> for Classes {
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
            Height::Custom(value) => classes!(format!("height-fixed-{}", value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Width {
    None,
    #[default]
    Small,
    Medium,
    Large,
    Full,
    Screen,
    Auto,
    Min,
    Max,
    Fit,
    Inherit,
    Custom(u8)
}

impl From<&Width> for Classes {
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
            Width::Custom(value) => classes!(format!("width-fixed-{}", value)),
        }
    }
}
