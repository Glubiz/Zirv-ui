use crate::components::{
    hero::hero::Hero,
};

use zirv_ui::{use_toast, Toast, ToastType, ThemeToggle};
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    let toasts_manager = use_toast::<Toast>();
        
    let onclick = Callback::from(move |_| {
        toasts_manager.spawn(Toast::new(
            ToastType::Info,
            "Test",
            "Test 2",
        ));
    });

    html! {
        <>
            <Hero />
            <button {onclick}>{"Show Toast"}</button>
        </>
    }
}
