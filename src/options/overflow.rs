use yew::{classes, Classes};

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    #[default]
    Auto,
}

impl From<&Overflow> for Classes {
    fn from(overflow: &Overflow) -> Self {
        match overflow {
            Overflow::Visible => classes!("overflow-visible"),
            Overflow::Hidden => classes!("overflow-hidden"),
            Overflow::Scroll => classes!("overflow-scroll"),
            Overflow::Auto => classes!("overflow-auto"),
        }
    }
}

