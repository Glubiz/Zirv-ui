use crate::sections::text::TextComponentsSection;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container};

#[function_component(TextPage)]
pub fn text() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <TextComponentsSection />
            </Container>
        </div>
    }
}
