//! Toast Provider Component
//!
//! This module provides the `ToastProvider` component for the Yew framework. The `ToastProvider`
//! component is responsible for managing and displaying toast notifications within a specified
//! context. It leverages the `ToastManager` and `ToastsList` to handle the lifecycle of toasts and
//! the `NotifiableComponentFactory` to create the toast components.

use std::marker::PhantomData;

use yew::{
    classes,
    function_component,
    html,
    use_effect_with,
    use_reducer_eq,
    Callback,
    Children,
    ContextProvider,
    Html,
    Properties,
};

use super::{
    manager::{
        Action,
        ToastManager,
        ToastsList,
    },
    utils::{
        Notifiable,
        NotifiableComponentFactory,
    },
};

/// Properties for the `ToastProvider` component.
#[derive(Properties, PartialEq, Clone)]
pub struct ToastProviderProps<T: Notifiable + PartialEq, F: NotifiableComponentFactory<T> + PartialEq + Clone> {
    /// The child components to render within the provider.
    pub children: Children,
    /// The component creator used to create toast components.
    pub component_creator: F,
    /// A phantom data marker for the toast type.
    #[prop_or_default]
    pub _toast: PhantomData<T>,
}

/// The `ToastProvider` component.
///
/// The `ToastProvider` component is responsible for managing and displaying toast notifications
/// within a specified context. It leverages the `ToastManager` and `ToastsList` to handle the
/// lifecycle of toasts and the `NotifiableComponentFactory` to create the toast components.
///
/// # Properties
///
/// - `children`: The child components to render within the provider.
/// - `component_creator`: The component creator used to create toast components.
/// - `_toast`: A phantom data marker for the toast type.
#[function_component(ToastProvider)]
pub fn toast_provider<T: Notifiable + PartialEq + Clone, F: NotifiableComponentFactory<T> + PartialEq + Clone>(
    props: &ToastProviderProps<T, F>,
) -> Html {
    let toasts = use_reducer_eq(ToastsList::<T>::default);

    let manager = ToastManager { sender: Some(toasts.dispatcher()) };

    use_effect_with((!toasts.is_empty(), toasts.dispatcher()), |(is_active, sender)| {
        use gloo::timers::callback::Interval;

        let sender = sender.clone();
        let is_active = *is_active;

        let interval = Interval::new(ToastsList::<T>::TIME_TICK_MILLIS as u32, move || {
            if is_active {
                sender.dispatch(Action::Tick);
            }
        });

        move || drop(interval)
    });

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
