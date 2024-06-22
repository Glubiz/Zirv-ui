use crate::sections::introduction::Introduction;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container, Loader, Style, use_toast, Button, Toast, ToastType};

#[function_component(Index)]
pub fn index() -> Html {
    let toasts_manager = use_toast::<Toast>();

    let onclick = Callback::from(move |_| {
        toasts_manager.spawn(Toast::new(ToastType::Info, "Test", "Test 2"));
    });

    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <Introduction />
                <Loader style={Style::Spinner} />
                <Loader style={Style::Dots} />
                <Button onclick={onclick}>{"Show Toast"}</Button>

            </Container>
        </div>
    }
}
