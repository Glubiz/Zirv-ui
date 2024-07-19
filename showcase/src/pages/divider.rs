use crate::sections::divider::DividerSection;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container};

#[function_component(DividerPage)]
pub fn divider() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <DividerSection />
            </Container>
        </div>
    }
}
