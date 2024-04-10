use yew::{classes, function_component, html, Html};

#[function_component(Header)]
pub fn toast() -> Html {
    html! {
        <header class={classes!{"header"}}>
            {"ting"}
        </header>
    }
}