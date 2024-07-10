//! Menu Provider Component Module
//!
//! This module provides a `MenuProvider` component that manages the state of a menu
//! and makes it available to child components through Yew's context system.

use yew::{
    function_component,
    html,
    use_state,
    Callback,
    Children,
    ContextProvider,
    Html,
    Properties,
};

use crate::menu::use_menu::MenuState;

/// Properties for the MenuProvider component
#[derive(Clone, PartialEq, Properties)]
pub struct MenuProviderProps {
    /// Child components that will have access to the menu state
    pub children: Children,
}

/// MenuProvider Component
///
/// This component manages the state of a menu and provides it to its children
/// through Yew's context system. It allows for toggling the menu's open/closed state.
///
/// # Example
///
/// ```
/// use your_crate::MenuProvider;
///
/// html! {
///     <MenuProvider>
///         <YourApp />
///     </MenuProvider>
/// }
/// ```
#[function_component(MenuProvider)]
pub fn menu_provider(props: &MenuProviderProps) -> Html {
    // State to track whether the menu is open or closed
    let is_open = use_state(|| false);

    // Callback to toggle the menu state
    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    // Create the MenuState with the current state and toggle callback
    let menu_state = MenuState { is_open: *is_open, toggle };

    // Render the ContextProvider with the MenuState and children
    html! {
        <ContextProvider<MenuState> context={menu_state}>
            {props.children.clone()}
        </ContextProvider<MenuState>>
    }
}