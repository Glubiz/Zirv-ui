use yew::{classes, Classes};

pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
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