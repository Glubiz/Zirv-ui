use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

use crate::toast::utils::Notifiable;

use super::Toast;

#[derive(Properties, Clone, PartialEq)]
pub struct ToastComponentProps {
    pub toast: Toast,
    pub onclick: Callback<MouseEvent>,
    pub onenter: Callback<MouseEvent>,
    pub onleave: Callback<MouseEvent>,
}

#[function_component(ToastComponent)]
pub fn toast_component(props: &ToastComponentProps) -> Html {
    let title = &props.toast.title;
    let text = &props.toast.text;
    let toast_type = &props.toast.toast_type;

    let onclick = &props.onclick;
    let onenter = &props.onenter;
    let onleave = &props.onleave;

    let mut classes = vec![classes!("toast"), toast_type.into()];
    if props.toast.is_paused() {
        classes.push(classes!("paused"));
    }

    if let Some(additional_classes) = &props.toast.classes {
        classes.push(classes!(additional_classes));
    }

    html! {
        <div {onclick} onmouseenter={onenter} onmouseleave={onleave} class={classes}>
            <span class={classes!("toast-title")}>{title}</span>
            <span>{text}</span>
        </div>
    }
}
