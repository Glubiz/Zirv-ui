use yew::prelude::*;
use zirv_ui::{
    options::{
        flex::FlexJustify,
        size::{Height, Width, CustomType},
    },
    Image,
    Container,
    MenuButton
};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <Container flex_justify={FlexJustify::SpaceBetween}>
                <Image src="../images/logo.png" alt="Logo" height={Height::Custom(4, CustomType::Fixed)} width={Width::Custom(4, CustomType::Fixed)} />
                <MenuButton />
            </Container>
        </>
    }
}
