use yew::{function_component, html, Html};
use zirv_ui::{Headline, Paragraph};

#[function_component(Introduction)]
pub fn introduction() -> Html {
    html! {
        <section>
            <Headline>{"Introduction"}</Headline>
            <Paragraph>{"This is a simple example of a Yew application using the yew_nested_router crate."}</Paragraph>
            <Paragraph>{"The application is structured as follows:"}</Paragraph>
            <ul>
                <li>{"components: Contains the reusable components of the application."}</li>
                <li>{"pages: Contains the pages of the application."}</li>
                <li>{"router: Contains the router configuration of the application."}</li>
                <li>{"sections: Contains the sections of the application."}</li>
            </ul>
            <Paragraph>{"The application is a simple example of a single page application (SPA) with a header and a router."}</Paragraph>
            <Paragraph>{"The header contains a link to the home page and a link to the introduction page."}</Paragraph>
            <Paragraph>{"The router is configured to render the home page by default and to render the introduction page when the introduction route is accessed."}</Paragraph>
            <Paragraph>{"The application also uses the zirv_ui crate to display toasts."}</Paragraph>
        </section>
    }
}
