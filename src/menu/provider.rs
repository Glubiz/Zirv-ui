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

#[derive(Clone, PartialEq, Properties)]
pub struct MenuProviderProps {
    pub children: Children,
}

#[function_component(MenuProvider)]
pub fn menu_provider(props: &MenuProviderProps) -> Html {
    let is_open = use_state(|| false);

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let menu_state = MenuState { is_open: *is_open, toggle };

    html! {
        <ContextProvider<MenuState> context={menu_state}>
            {props.children.clone()}
        </ContextProvider<MenuState>>
    }
}
