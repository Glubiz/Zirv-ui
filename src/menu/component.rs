use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

use super::message::Menu;

#[derive(Properties, Clone, PartialEq)]
pub struct MenuComponentProps {
    pub menu: Menu,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(MenuComponent)]
pub fn menu_component(props: &MenuComponentProps) -> Html {
    let onclick = &props.onclick;

    let mut classes = vec![classes!("menu")];

    if let Some(additional_classes) = &props.menu.classes {
        classes.push(classes!(additional_classes));
    }

    html! {
        <div {onclick} class={classes}>
            {props.menu.children.clone()}
        </div>
    }
}
