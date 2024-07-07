use crate::sections::button::ButtonSection;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container};

#[function_component(ButtonPage)]
pub fn button() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <ButtonSection />
            </Container>
        </div>
    }
}
