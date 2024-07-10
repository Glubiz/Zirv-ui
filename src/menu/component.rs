//! Menu Component Module
//!
//! This module provides a `Menu` component that renders a list of menu items.
//! The menu can be toggled open or closed, and its state is managed by the `use_menu` hook.

use yew::prelude::*;

use crate::menu::use_menu::use_menu;

/// Represents a single menu item with text and a URL.
#[derive(PartialEq)]
pub struct MenuItem {
    /// The text to display for the menu item
    pub text: String,
    /// The URL that the menu item links to
    pub url: String,
}

/// Properties for the Menu component
#[derive(Properties, PartialEq)]
pub struct MenuProps {
    /// A vector of MenuItem structs representing the menu items to be displayed
    pub items: Vec<MenuItem>,
}

/// Menu Component
///
/// Renders a navigation menu with a list of items. The menu can be toggled open or closed.
///
/// # Props
///
/// * `items` - A vector of `MenuItem` structs representing the menu items to be displayed
///
/// # Example
///
/// ```
/// use your_crate::{
///     Menu,
///     MenuItem,
/// };
///
/// let menu_items = vec![
///     MenuItem { text: "Home".to_string(), url: "/".to_string() },
///     MenuItem { text: "About".to_string(), url: "/about".to_string() },
/// ];
///
/// html! {
///     <Menu items={menu_items} />
/// }
/// ```
#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    // Get the current menu state using the use_menu hook
    let menu_state = use_menu();

    // Determine CSS classes based on whether the menu is open or closed
    let menu_class = if menu_state.is_open { "menu open" } else { "menu" };
    let backdrop_class = if menu_state.is_open { "menu-backdrop open" } else { "menu-backdrop" };

    html! {
        <>
            // Render a backdrop that can be clicked to close the menu
            <div class={backdrop_class} onclick={menu_state.toggle.clone()}></div>
            // Render the navigation menu
            <nav class={menu_class}>
                <ul>
                    // Iterate over menu items and render each as a list item
                    { for props.items.iter().map(|item| html! {
                        <li><a href={item.url.clone()}>{ &item.text }</a></li>
                    }) }
                </ul>
            </nav>
        </>
    }
}
