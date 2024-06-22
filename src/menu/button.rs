use yew::prelude::*;

use crate::menu::use_menu::MenuState;

#[function_component(MenuButton)]
pub fn menu_button() -> Html {
    let menu_state = use_context::<MenuState>().expect("No context found!");

    let button_class = if menu_state.is_open { "open" } else { "" };

    html! {
        <button class={classes!("menu-button", button_class)} onclick={menu_state.toggle.clone()}>
            <div class="bar1"></div>
            <div class="bar2"></div>
            <div class="bar3"></div>
        </button>
    }
}
