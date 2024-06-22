mod components;
mod pages;
mod router;
mod sections;

use wasm_bindgen::prelude::*;
use yew::{function_component, html, Html};
use yew_router::{Routable, switch:: Switch, router::BrowserRouter};

use components::header::Header;
use zirv_ui::{Toast, ToastFactory, ToastProvider, MenuItem, MenuProvider, Menu};

use crate::router::{Route, switch};


pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<Application>::new().render();
    Ok(())
}

#[function_component(Application)]
pub fn app() -> Html {
    let component_creator = ToastFactory;

    let menu_items = vec![
        MenuItem {
            text: "Home".to_string(),
            url: Route::Index.to_path(),
        },
        MenuItem {
            text: "Getting Started".to_string(),
            url: Route::GettingStarted.to_path(),
        },
    ];

    html! {
        <ToastProvider<Toast, ToastFactory> {component_creator}>
            <MenuProvider>
                <Header/>
                <Menu items={menu_items} />
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </MenuProvider>
        </ToastProvider<Toast, ToastFactory>>
    }
}
