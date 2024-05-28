mod components;
mod pages;
mod router;
mod sections;

use wasm_bindgen::prelude::*;
use yew::{function_component, html, Html};
use yew_nested_router::prelude::{Switch as RouterSwitch, *};

use crate::router::{switch_app_route, AppRoute};
use components::header::Header;
use zirv_ui::{Toast, ToastFactory, ToastProvider};

#[cfg(debug_assertions)]
const LOG_LEVEL: log::Level = log::Level::Trace;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(LOG_LEVEL));
    yew::Renderer::<Application>::new().render();
    Ok(())
}

#[function_component(Application)]
pub fn app() -> Html {
    let component_creator = ToastFactory;

    html! {
        <ToastProvider<Toast, ToastFactory> {component_creator}>
            <Header/>
            <Router<AppRoute> default={AppRoute::Index}>
                <RouterSwitch<AppRoute> render={switch_app_route} />
            </Router<AppRoute>>
        </ToastProvider<Toast, ToastFactory>>
    }
}
