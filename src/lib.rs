mod options;
mod container;
mod flex;
mod text;
mod toast;

#[cfg(feature = "container")]
pub use container::{Container, ContainerProps};

#[cfg(feature = "flex")]
pub use flex::{Flex, FlexProps};

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
