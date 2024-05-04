use zirv_ui::Container;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {

    html! {
        <>
            <Container classes={classes!("p-4")}>
                <h1>{"Hello, World!"}</h1>
            </Container>
        </>
    }
}
