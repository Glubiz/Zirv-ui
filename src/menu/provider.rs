use std::marker::PhantomData;

use yew::{function_component, html, Callback, Children, ContextProvider, Html, Properties};

use super::manager::MenuManager;

#[derive(Properties, PartialEq, Clone)]
pub struct MenuProviderProps<T: PartialEq + Clone, F: PartialEq + Clone> {
    pub children: Children,
    pub component_creator: F,
    #[prop_or_default]
    pub _menu: PhantomData<T>,
}

#[function_component(MenuProvider)]
pub fn menu_provider<T: PartialEq + Clone, F: PartialEq + Clone>(props: &MenuProviderProps<T, F>) -> Html {
    let manager = MenuManager {
        sender: Some(menu.dispatcher()),
    };

    let dispatcher = toasts.dispatcher();

    let creator = &props.component_creator;

    html! {
        <ContextProvider<ManuManager<T>> context={manager}>
            <div class={classes!("menu")}>
                let onclick = {
                    let dispatcher = dispatcher.clone();
                    Callback::from(move |_| {
                        dispatcher.dispatch(Action::Close(id));
                    })
                };

                creator.component(toast, onclick)
            </div>
        </ContextProvider<ToastManager<T>>>
    }
}
