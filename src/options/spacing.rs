use yew::{classes, Classes};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Padding {
    None,
    #[default]
    Small,
    Medium,
    Large,
}

impl From<&Padding> for Classes {
    fn from(padding: &Padding) -> Self {
        match padding {
            Padding::None => classes!("padding-none"),
            Padding::Small => classes!("padding-small"),
            Padding::Medium => classes!("padding-medium"),
            Padding::Large => classes!("padding-large"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Margin {
    None,
    #[default]
    Small,
    Medium,
    Large,
}

impl From<&Margin> for Classes {
    fn from(margin: &Margin) -> Self {
        match margin {
            Margin::None => classes!("margin-none"),
            Margin::Small => classes!("margin-small"),
            Margin::Medium => classes!("margin-medium"),
            Margin::Large => classes!("margin-large"),
        }
    }
}
