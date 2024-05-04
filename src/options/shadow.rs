use yew::{classes, Classes};

pub enum BoxShadow {
    None,
    Small,
    Medium,
    Large,
}

impl From<&BoxShadow> for Classes {
    fn from(box_shadow: &BoxShadow) -> Self {
        match box_shadow {
            BoxShadow::None => classes!("box-shadow-none"),
            BoxShadow::Small => classes!("box-shadow-small"),
            BoxShadow::Medium => classes!("box-shadow-medium"),
            BoxShadow::Large => classes!("box-shadow-large"),
        }
    }
}