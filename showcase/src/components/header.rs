use zirv_ui::{use_toast, Toast, ToastType, Container};
use yew::prelude::*;
use zirv_ui::options::color::BackgroundColor;

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
            <Container background_color={BackgroundColor::Container}>
                <img src="../images/logo.png" alt="Logo" class="h-12 w-12" />
                <button {onclick}>{"Show Toast"}</button>
            </Container>
        </>
    }
}
