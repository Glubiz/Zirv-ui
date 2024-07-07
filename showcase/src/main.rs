mod components;
mod pages;
mod router;
mod sections;

use wasm_bindgen::prelude::*;
use yew::{function_component, html, Html};
use yew_router::{Routable, switch:: Switch, router::BrowserRouter};

use components::header::Header;
use zirv_ui::{Toast, ToastFactory, ToastProvider, MenuItem, MenuProvider, Menu, ThemeProvider, Theme};

use crate::router::{Route, switch};


pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<Application>::new().render();
    Ok(())
}

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
        MenuItem {
            text: "Button".to_string(),
            url: Route::Button.to_path(),
        },
        MenuItem {
            text: "Loader".to_string(),
            url: Route::Loader.to_path(),
        },
        MenuItem {
            text: "Table".to_string(),
            url: Route::Table.to_path(),
        },
        MenuItem {
            text: "Text".to_string(),
            url: Route::Text.to_path(),
        },
        MenuItem {
            text: "Toast".to_string(),
            url: Route::Toast.to_path(),
        },
        MenuItem {
            text: "Container".to_string(),
            url: Route::Container.to_path(),
        },
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
