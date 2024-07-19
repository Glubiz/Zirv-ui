//! Menu Component Module
//!
//! This module provides a `Menu` component that renders a list of menu items.
//! The menu can be toggled open or closed, and its state is managed by the `use_menu` hook.
use yew::prelude::*;

use crate::menu::use_menu::use_menu;

#[derive(Clone, PartialEq)]
pub struct Drawer {
    pub name: String,
    pub is_open: bool,
}

/// Represents a single menu item with text and a URL.
#[derive(PartialEq, Clone)]
pub struct MenuItem {
    /// The text to display for the menu item
    pub text: String,
    /// The URL that the menu item links to
    pub url: String,
}

#[derive(Clone, PartialEq)]
pub struct Section {
    pub name: String,
    pub items: Vec<MenuItem>,
    pub is_open: bool,
}

impl Section {
    pub fn toggle(self) -> Self {
        Self { is_open: !self.is_open, ..self }
    }
}

#[derive(Clone, PartialEq)]
pub enum MenuEntry {
    Item(MenuItem),
    Section(Section),
}

/// Properties for the Menu component
#[derive(Properties, PartialEq)]
pub struct MenuProps {
    pub items: Vec<MenuEntry>,
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
    let menu_state = use_menu();
    let menu_class = if menu_state.is_open { "menu open" } else { "menu" };
    let backdrop_class = if menu_state.is_open { "menu-backdrop open" } else { "menu-backdrop" };

    let menu_entries = use_state(|| props.items.clone());

    let toggle_section = {
        let menu_entries = menu_entries.clone();
        Callback::from(move |index: usize| {
            menu_entries.set(
                menu_entries
                    .iter()
                    .enumerate()
                    .map(|(i, entry)| {
                        if i == index {
                            match entry {
                                MenuEntry::Section(section) => MenuEntry::Section(section.clone().toggle()),
                                _ => entry.clone(),
                            }
                        } else {
                            entry.clone()
                        }
                    })
                    .collect(),
            );
        })
    };

    html! {
        <>
            <div class={backdrop_class} onclick={menu_state.toggle.clone()}></div>
            <nav class={menu_class}>
                <ul>
                    {
                        for menu_entries.iter().enumerate().map(|(index, entry)| match entry {
                            MenuEntry::Item(item) => html! {
                                <li><a href={item.url.clone()}>{ &item.text }</a></li>
                            },
                            MenuEntry::Section(section) => {
                                html! {
                                    <li class={classes!("section", section.is_open.then_some("open"))}>
                                        <div
                                            class="section-header"
                                            onclick={toggle_section.reform(move |_| index)}
                                        >
                                            { &section.name }
                                            <span class="section-icon">{ "▼" }</span>
                                        </div>
                                        <ul class="section-items">
                                            { for section.items.iter().map(|item| html! {
                                                <li><a href={item.url.clone()}>{ &item.text }</a></li>
                                            }) }
                                        </ul>
                                    </li>
                                }
                            }
                        })
                    }
                </ul>
            </nav>
        </>
    }
}
