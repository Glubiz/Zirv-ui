use crate::sections::toast::ToastSection;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container};

#[function_component(ToastPage)]
pub fn toast() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <ToastSection />
            </Container>
        </div>
    }
}
