use yew::{classes, function_component, html, Callback, Html, use_context};

use super::provider::ThemeContext;

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let theme_context = use_context::<ThemeContext>().expect("no ctx found");

    let onclick = {
        let theme_context = theme_context.clone();
        Callback::from(move |_| {
            theme_context.toggle_theme.emit(());
        })
    };

    html! {
        <div>
            <button {onclick}>{ "Toggle Theme" }</button>
            <p>{ format!("Current theme: {:?}", theme_context.theme) }</p>
        </div>
    }
}