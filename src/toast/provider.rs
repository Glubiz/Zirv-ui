use std::marker::PhantomData;

use yew::{
    classes, function_component, html, use_effect_with, use_reducer_eq, Callback, Children,
    ContextProvider, Html, Properties,
};

use super::manager::{Action, ToastManager, ToastsList};
use super::utils::{Notifiable, NotifiableComponentFactory};

#[derive(Properties, PartialEq, Clone)]
pub struct ToastProviderProps<
    T: Notifiable + PartialEq,
    F: NotifiableComponentFactory<T> + PartialEq + Clone,
> {
    pub children: Children,
    pub component_creator: F,
    #[prop_or_default]
    pub _toast: PhantomData<T>,
}

#[function_component(ToastProvider)]
pub fn toast_provider<
    T: Notifiable + PartialEq + Clone,
    F: NotifiableComponentFactory<T> + PartialEq + Clone,
>(
    props: &ToastProviderProps<T, F>,
) -> Html {
    let toasts = use_reducer_eq(ToastsList::<T>::default);

    let manager = ToastManager {
        sender: Some(toasts.dispatcher()),
    };

    use_effect_with(
        (!toasts.is_empty(), toasts.dispatcher()),
        |(is_active, sender)| {
            use gloo::timers::callback::Interval;

            let sender = sender.clone();
            let is_active = *is_active;

            let interval = Interval::new(ToastsList::<T>::TIME_TICK_MILLIS as u32, move || {
                if is_active {
                    sender.dispatch(Action::Tick);
                }
            });

            move || drop(interval)
        },
    );

    let children = props.children.clone();
    let dispatcher = toasts.dispatcher();

    let toast_creator = &props.component_creator;

    html! {
        <ContextProvider<ToastManager<T>> context={manager}>
            {children}
            <div class={classes!("toasts", "toasts-provider-bottom-right")}>
                {for toasts.toasts.iter().map(|t| {
                    let toast = t.clone();
                    let id = toast.id();

                    let onclick = {
                        let dispatcher = dispatcher.clone();
                        Callback::from(move |_| {
                            dispatcher.dispatch(Action::Close(id));
                        })
                    };

                    let onenter = {
                        let dispatcher = dispatcher.clone();
                        Callback::from(move |_| {
                            dispatcher.dispatch(Action::Pause(id));
                        })
                    };

                    let onleave = {
                        let dispatcher = dispatcher.clone();
                        Callback::from(move |_| {
                            dispatcher.dispatch(Action::Continue(id));
                        })
                    };

                    toast_creator.component(toast, onclick, onenter, onleave)
                })}
            </div>
        </ContextProvider<ToastManager<T>>>
    }
}
