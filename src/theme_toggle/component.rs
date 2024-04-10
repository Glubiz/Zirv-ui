use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

pub fn toggle_theme(theme: Theme) {
    if let Some(window) = window() {
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let body: HtmlElement = body.dyn_into::<HtmlElement>().unwrap();
        
        match theme {
            Theme::Dark => {
                body.set_class_name("dark-theme");
            },
            Theme::Light => {
                body.set_class_name("light-theme");
            },
        }
    }
}