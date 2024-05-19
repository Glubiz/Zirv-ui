use zirv_ui::{use_toast, Toast, ToastType, Container, Button};
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
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
            <Container>
                <img src="../images/logo.png" alt="Logo" class="h-12 w-12" />
                <Button onclick={onclick}>{"Show Toast"}</Button>
            </Container>
        </>
    }
}
