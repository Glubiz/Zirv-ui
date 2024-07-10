use yew::{function_component, html, Html};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline, Paragraph, CodeBlock, Subheadline
};

#[function_component(GettingStartedSection)]
pub fn getting_started() -> Html {
    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::Large}>
                <Headline>{"Getting Started with Zirv UI"}</Headline>

                <Subheadline>{"Installation"}</Subheadline>
                <Paragraph>{"To get started with Zirv UI, you can install the package using cargo:"}</Paragraph>
                <CodeBlock 
                    snippet={"cargo add zirv_ui"}
                    language="Shell"
                />

                <Subheadline>{"Adding Styles"}</Subheadline>
                <Paragraph>{"After installing the package, you need to add the styling to the index.html. Add the following snippet to the head:"}</Paragraph>
                <CodeBlock 
                    snippet={"<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/gh/Glubiz/Zirv-ui@main/style/dist/main.css\">"}
                    language="HTML"
                />

                <Subheadline>{"Project Setup"}</Subheadline>
                <Paragraph>{"To set up your Zirv UI project, you'll need to structure your main.rs file. Here's a guide on how to do this:"}</Paragraph>

                <Subheadline>{"1. Import Necessary Modules and Components"}</Subheadline>
                <CodeBlock 
                    snippet={r#"
mod components;
mod pages;
mod router;
mod sections;

use wasm_bindgen::prelude::*;
use yew::{function_component, html, Html};
use yew_router::{Routable, switch::Switch, router::BrowserRouter};
use zirv_ui::{Toast, ToastFactory, ToastProvider, MenuItem, MenuProvider, Menu, ThemeProvider, Theme};

use components::header::Header;
use crate::router::{Route, switch};
                    "#}
                    language="Rust"
                />

                <Subheadline>{"2. Create the Main Function"}</Subheadline>
                <CodeBlock 
                    snippet={r#"
pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<Application>::new().render();
    Ok(())
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"3. Define the Application Component"}</Subheadline>
                <CodeBlock 
                    snippet={r#"
#[function_component(Application)]
pub fn app() -> Html {
    let component_creator = ToastFactory;
    let theme = Theme::default();

    let menu_items = vec![
        MenuItem {
            text: "Home".to_string(),
            url: Route::Index.to_path(),
        },
        MenuItem {
            text: "Getting Started".to_string(),
            url: Route::GettingStarted.to_path(),
        },
        // Add more menu items as needed
    ];

    html! {
        <ThemeProvider theme={theme}>
            <ToastProvider<Toast, ToastFactory> {component_creator}>
                <MenuProvider>
                    <Header/>
                    <Menu items={menu_items} />
                    <BrowserRouter>
                        <Switch<Route> render={switch} />
                    </BrowserRouter>
                </MenuProvider>
            </ToastProvider<Toast, ToastFactory>>
        </ThemeProvider>
    }
}
                    "#}
                    language="Rust"
                />

                <Paragraph>{"This structure sets up:"}</Paragraph>
                <ul>
                    <li>{"A ThemeProvider for consistent styling"}</li>
                    <li>{"A ToastProvider for displaying notifications"}</li>
                    <li>{"A MenuProvider and Menu component for navigation"}</li>
                    <li>{"A BrowserRouter and Switch for handling routing in your application"}</li>
                </ul>

                <Subheadline>{"Using Components"}</Subheadline>
                <Paragraph>{"Once you've set up your project, you can start using Zirv UI components. Import them like this:"}</Paragraph>
                <CodeBlock 
                    snippet={"use zirv_ui::Button;"}
                    language="Rust"
                />
                <Paragraph>{"Then use them in your components:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
#[function_component(MyComponent)]
pub fn my_component() -> Html {
    html! {
        <Button onclick={Callback::from(|_| log::info!("Button clicked"))}>
            {"Click me"}
        </Button>
    }
}
                    "#}
                    language="Rust"
                />

                <Paragraph>{"Remember to define your routes in the router.rs file and create the corresponding page components. The switch function in your router will handle rendering the appropriate component based on the current route."}</Paragraph>
                <Paragraph>{"This setup provides a solid foundation for building complex applications with Zirv UI, leveraging its various components and features within a well-structured Yew application."}</Paragraph>
            </Container>
        </section>
    }
}