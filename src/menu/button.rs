//! Menu Button Component Module
//!
//! This module provides a `MenuButton` component that toggles a menu's open/closed state.
//! It uses Yew's context system to access and modify the menu state.

use yew::prelude::*;

use crate::menu::use_menu::MenuState;

/// MenuButton Component
///
/// Renders a button that toggles the menu open/closed state.
/// The button's appearance changes based on the menu state.
///
/// # Context Requirements
///
/// This component expects a `MenuState` to be available in the Yew context.
/// It will panic if the context is not found.
///
/// # Example
///
/// ```
/// use your_crate::{
///     menu::use_menu::MenuState,
///     MenuButton,
/// };
///
/// html! {
///     <ContextProvider<MenuState> context={MenuState::default()}>
///         <MenuButton />
///     </ContextProvider<MenuState>>
/// }
/// ```
#[function_component(MenuButton)]
pub fn menu_button() -> Html {
    // Retrieve the MenuState from the Yew context
    let menu_state = use_context::<MenuState>().expect("No context found!");

    // Determine the button class based on the menu state
    let button_class = if menu_state.is_open { "open" } else { "" };

    // Render the menu button
    html! {
        <button class={classes!("menu-button", button_class)} onclick={menu_state.toggle.clone()}>
            <div class="bar1"></div>
            <div class="bar2"></div>
            <div class="bar3"></div>
        </button>
    }
}
