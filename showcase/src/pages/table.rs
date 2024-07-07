use crate::sections::table::TableSection;
use yew::prelude::*;
use zirv_ui::{options::flex::FlexDirection, Container};

#[function_component(TablePage)]
pub fn table() -> Html {
    html! {
        <div>
            <Container flex_direction={FlexDirection::Column}>
                <TableSection />
            </Container>
        </div>
    }
}
