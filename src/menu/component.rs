use yew::prelude::*;

use crate::menu::use_menu::MenuState;

#[derive(PartialEq)]
pub struct MenuItem {
    pub text: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    pub items: Vec<MenuItem>,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let menu_state = use_context::<MenuState>().expect("No context found!");

    let menu_class = if menu_state.is_open { "menu open" } else { "menu" };
    let backdrop_class = if menu_state.is_open { "menu-backdrop open" } else { "menu-backdrop" };

    html! {
        <>
            <div class={backdrop_class} onclick={menu_state.toggle.clone()}></div>
            <nav class={menu_class}>
                <ul>
                    { for props.items.iter().map(|item| html! {
                        <li><a href={item.url.clone()}>{ &item.text }</a></li>
                    }) }
                </ul>
            </nav>
        </>
    }
}
