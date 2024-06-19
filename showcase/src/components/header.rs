use yew::prelude::*;
use zirv_ui::{
    options::{
        flex::FlexJustify,
        size::{Height, Width, CustomType},
    },
    use_toast, Button, Container, Image, Toast, ToastType,
};

#[function_component(Header)]
pub fn header() -> Html {
    let toasts_manager = use_toast::<Toast>();

    let onclick = Callback::from(move |_| {
        toasts_manager.spawn(Toast::new(ToastType::Info, "Test", "Test 2"));
    });

    html! {
        <>
            <Container flex_justify={FlexJustify::SpaceBetween}>
                <Image src="../images/logo.png" alt="Logo" height={Height::Custom(4, CustomType::Fixed)} width={Width::Custom(4, CustomType::Fixed)} />
                <Button onclick={onclick}>{"Show Toast"}</Button>
            </Container>
        </>
    }
}
