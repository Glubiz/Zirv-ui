use crate::sections::theme::ThemeSection;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container};

#[function_component(ThemePage)]
pub fn theme() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <ThemeSection />
            </Container>
        </div>
    }
}
