pub mod button;
pub mod container;
pub mod flex;
// pub mod list;
pub mod loader;
// pub mod menu;
pub mod image;
pub mod options;
pub mod table;
pub mod text;
pub mod toast;

pub use options::*;

#[cfg(feature = "button")]
pub use button::{Button, ButtonProps};

#[cfg(feature = "container")]
pub use container::{Container, ContainerProps};

#[cfg(feature = "flex")]
pub use flex::{Flex, FlexProps};

// #[cfg(feature = "list")]
// pub use list::{List, ListProps};

#[cfg(feature = "loader")]
pub use loader::{Loader, LoaderProps, Style};

#[cfg(feature = "table")]
pub use table::{Table, TableProps};

// #[cfg(feature = "menu")]
// pub use menu::{
//     component::{MenuComponent, MenuComponentProps},
//     manager::MenuManager,
//     provider::{MenuProvider, MenuProviderProps},
//     use_menu::use_menu,
//     utils::MenuType,
//     Menu,
// };

#[cfg(feature = "text")]
pub use text::{
    headline::{Headline, HeadlineProps},
    paragraph::{Paragraph, ParagraphProps},
    subheadline::{Subheadline, SubheadlineProps},
};

#[cfg(feature = "toast")]
pub use toast::{
    component::{ToastComponent, ToastComponentProps},
    component_factory::ToastFactory,
    manager::ToastManager,
    provider::{ToastProvider, ToastProviderProps},
    use_toast::use_toast,
    utils::ToastType,
    Toast,
};

#[cfg(feature = "image")]
pub use image::{Image, ImageProps};
