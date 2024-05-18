use zirv_ui::Container;
use yew::prelude::*;
use zirv_ui::options::color::BackgroundColor;

#[function_component(Index)]
pub fn index() -> Html {

    html! {
        <>
            <Container background_color={BackgroundColor::Container}>
                <h1>{"Hello, World!"}</h1>
            </Container>
        </>
    }
}
