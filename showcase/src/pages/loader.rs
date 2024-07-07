use crate::sections::loader::LoaderSection;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container};

#[function_component(LoaderPage)]
pub fn loader() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <LoaderSection />
            </Container>
        </div>
    }
}
