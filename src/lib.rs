//! Component Library
//!
//! This module serves as the main entry point for the component library. It organizes and exports
//! various UI components and utilities, which can be optionally included based on feature flags.
//! The feature flags allow for selective inclusion of components to reduce the overall bundle size
//! and dependencies.
//!
//! # Example
//!
//! To use the components in your Yew application, enable the appropriate features in your
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies.zirv_ui]
//! version = "0.1.0"
//! features = ["button", "container", "flex", "loader", "table", "text", "toast", "image"]
//! ```
//!
//! Then, import and use the components as needed:
//!
//! ```rust
//! use yew::{
//!     html,
//!     Callback,
//!     Html,
//! };
//! use zirv_ui::{
//!     Button,
//!     ButtonProps,
//!     Container,
//!     ContainerProps,
//! };
//!
//! fn view() -> Html {
//!     html! {
//!         <>
//!             <Button onclick={Callback::from(|_| log::info!("Button clicked"))}>
//!                 {"Click me"}
//!             </Button>
//!             <Container>
//!                 {"This is a container"}
//!             </Container>
//!         </>
//!     }
//! }
//! ```

pub mod button;
pub mod code_block;
pub mod container;
pub mod divider;
pub mod flex;
pub mod image;
pub mod list;
pub mod loader;
pub mod menu;
pub mod options;
pub mod table;
pub mod text;
pub mod theme;
pub mod toast;

#[cfg(feature = "button")]
pub use button::{
    Button,
    ButtonProps,
};
#[cfg(feature = "code_block")]
pub use code_block::{
    CodeBlock,
    CodeBlockProps,
};
#[cfg(feature = "container")]
pub use container::{
    Container,
    ContainerProps,
};
#[cfg(feature = "divider")]
pub use divider::{
    Divider,
    DividerProps,
};
#[cfg(feature = "flex")]
pub use flex::{
    Flex,
    FlexProps,
};
#[cfg(feature = "image")]
pub use image::{
    Image,
    ImageProps,
};
#[cfg(feature = "list")]
pub use list::{
    List,
    ListProps,
};
#[cfg(feature = "loader")]
pub use loader::{
    Loader,
    LoaderProps,
    Style,
};
#[cfg(feature = "menu")]
pub use menu::{
    button::MenuButton,
    component::{
        Menu,
        MenuEntry,
        MenuItem,
        MenuProps,
        Section,
    },
    provider::{
        MenuProvider,
        MenuProviderProps,
    },
    use_menu::use_menu,
};
pub use options::*;
#[cfg(feature = "table")]
pub use table::{
    Table,
    TableProps,
};
#[cfg(feature = "text")]
pub use text::{
    headline::{
        Headline,
        HeadlineProps,
    },
    paragraph::{
        Paragraph,
        ParagraphProps,
    },
    subheadline::{
        Subheadline,
        SubheadlineProps,
    },
};
#[cfg(feature = "theme")]
pub use theme::{
    Theme,
    ThemeProps,
    ThemeProvider,
};
#[cfg(feature = "toast")]
pub use toast::{
    component::{
        ToastComponent,
        ToastComponentProps,
    },
    component_factory::ToastFactory,
    manager::ToastManager,
    provider::{
        ToastProvider,
        ToastProviderProps,
    },
    use_toast::use_toast,
    utils::ToastType,
    Toast,
};
