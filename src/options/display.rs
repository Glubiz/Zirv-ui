use yew::{classes, Classes};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Display {
    None,
    Block,
    Inline,
    InlineBlock,
    #[default]
    Flex,
    Grid,
}

impl From<&Display> for Classes {
    fn from(display: &Display) -> Self {
        match display {
            Display::None => classes!("display-none"),
            Display::Block => classes!("display-block"),
            Display::Inline => classes!("display-inline"),
            Display::InlineBlock => classes!("display-inline-block"),
            Display::Flex => classes!("display-flex"),
            Display::Grid => classes!("display-grid"),
        }
    }
}
