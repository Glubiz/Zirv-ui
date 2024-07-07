use crate::sections::container::ContainerSection;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container};

#[function_component(ContainerPage)]
pub fn container() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <ContainerSection />
            </Container>
        </div>
    }
}
