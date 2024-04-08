use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub message: String,
    pub toast_type: String,
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    html! {
        <div class={format!("toast {}", props.toast_type)}>
            { &props.message }
        </div>
    }
}