use crate::sections::getting_started::GettingStartedSection;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container};

#[function_component(GettingStartedPage)]
pub fn getting_started() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <GettingStartedSection />
            </Container>
        </div>
    }
}
