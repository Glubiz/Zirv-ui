use yew::{classes, Classes};

#[derive(Clone, PartialEq, Default)]
pub enum ZIndex {
    #[default]
    None,
    Front,
    Middle,
    Back,
}

impl From<&ZIndex> for Classes {
    fn from(z_index: &ZIndex) -> Self {
        match z_index {
            ZIndex::None => classes!("z-index-none"),
            ZIndex::Front => classes!("z-index-front"),
            ZIndex::Middle => classes!("z-index-middle"),
            ZIndex::Back => classes!("z-index-back"),
        }
    }
}

