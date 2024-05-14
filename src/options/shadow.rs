use yew::{classes, Classes};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BoxShadow {
    #[default]
    None,
    Some,
}

impl From<&BoxShadow> for Classes {
    fn from(box_shadow: &BoxShadow) -> Self {
        match box_shadow {
            BoxShadow::None => classes!("box-shadow-none"),
            BoxShadow::Some => classes!("box-shadow")
        }
    }
}