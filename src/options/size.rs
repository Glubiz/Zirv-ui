use yew::{classes, Classes};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Height {
    None,
    #[default]
    Small,
    Medium,
    Large,
    Full,
}

impl From<&Height> for Classes {
    fn from(height: &Height) -> Self {
        match height {
            Height::None => classes!("height-none"),
            Height::Small => classes!("height-small"),
            Height::Medium => classes!("height-medium"),
            Height::Large => classes!("height-large"),
            Height::Full => classes!("height-full"),
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
}

impl From<&Width> for Classes {
    fn from(width: &Width) -> Self {
        match width {
            Width::None => classes!("width-none"),
            Width::Small => classes!("width-small"),
            Width::Medium => classes!("width-medium"),
            Width::Large => classes!("width-large"),
            Width::Full => classes!("width-full"),
        }
    }
}
