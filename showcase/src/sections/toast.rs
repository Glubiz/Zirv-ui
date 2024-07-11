use yew::{function_component, html, Html, Callback};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline, Paragraph, CodeBlock, Subheadline, Toast, ToastType, use_toast,
};

#[function_component(ToastSection)]
pub fn toast_section() -> Html {
    let toasts_manager = use_toast::<Toast>();

    let onclick = Callback::from(move |_| {
        toasts_manager.spawn(Toast::new(
            ToastType::Info,
            "Info",
            "This is an informational message."
        ));
    });

    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::Large}>
                <Headline>{"Toast System"}</Headline>

                <Paragraph>{"The Toast system in Zirv UI provides a flexible and customizable way to display toast notifications in Yew applications. It includes components for creating, managing, and displaying toasts, as well as utilities for easy integration into your application."}</Paragraph>

                <Subheadline>{"Basic Usage"}</Subheadline>
                <Paragraph>{"Here's a simple example of how to use the Toast system:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
use yew::{function_component, html, Html, Callback};
use zirv_ui::{Toast, ToastType, ToastFactory, ToastProvider, use_toast};

#[function_component(App)]
fn app() -> Html {
    let toast_manager = use_toast::<Toast>();

    let onclick = Callback::from(move |_| {
        toast_manager.spawn(Toast::new(
            ToastType::Info,
            "Info",
            "This is an informational message."
        ));
    });

    html! {
        <ToastProvider<Toast, ToastFactory> component_creator={ToastFactory}>
            <button {onclick}>{"Show Toast"}</button>
        </ToastProvider<Toast, ToastFactory>>
    }
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Key Components"}</Subheadline>
                <Paragraph>{"The Toast system consists of several key components:"}</Paragraph>
                <ul>
                    <li>{"Toast: Represents a single toast notification."}</li>
                    <li>{"ToastManager: Manages the creation and dispatching of toast notifications."}</li>
                    <li>{"ToastsList: Handles the collection and state updates of toasts."}</li>
                    <li>{"ToastProvider: A Yew component that provides the context for managing toasts."}</li>
                    <li>{"ToastComponent: A Yew component for rendering individual toast notifications."}</li>
                    <li>{"ToastFactory: Creates ToastComponent instances."}</li>
                    <li>{"use_toast: A custom hook for accessing the ToastManager in Yew components."}</li>
                </ul>

                <Subheadline>{"Toast Types"}</Subheadline>
                <Paragraph>{"The Toast system supports different types of notifications:"}</Paragraph>
                <ul>
                    <li>{"ToastType::Info: For general informational messages."}</li>
                    <li>{"ToastType::Warn: For warning messages."}</li>
                    <li>{"ToastType::Error: For error messages."}</li>
                </ul>

                <Subheadline>{"Customization Examples"}</Subheadline>
                
                <Paragraph>{"1. Creating a Toast with custom type and duration:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
let custom_toast = Toast::new(ToastType::Warn, "Warning", "This is a warning message.")
    .with_lifetime(Duration::milliseconds(5000));
toast_manager.spawn(custom_toast);
                    "#}
                    language="Rust"
                />

                <Paragraph>{"2. Using the ToastProvider with custom styling:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<ToastProvider<Toast, ToastFactory> 
    component_creator={ToastFactory}
    classes={classes!("custom-toast-container")}
>
    <App />
</ToastProvider<Toast, ToastFactory>>
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Live Example"}</Subheadline>
                <Paragraph>{"Here's a live example of the Toast system:"}</Paragraph>

                // Note: In a real implementation, you would need to set up the ToastProvider
                // and use the use_toast hook to make this example work.
                <button onclick={onclick}>
                    {"Show Toast"}
                </button>

                <Subheadline>{"Implementation Details"}</Subheadline>
                <Paragraph>{"The Toast system uses Yew's context system for global state management. Toasts are uniquely identified using UUIDs and managed using a tick-based approach for handling lifetimes and expiration. The ToastProvider component handles the rendering and positioning of toasts."}</Paragraph>

                <Subheadline>{"Best Practices"}</Subheadline>
                <Paragraph>{"When using the Toast system, consider the following best practices:"}</Paragraph>
                <ul>
                    <li>{"Use appropriate toast types for different kinds of messages (Info for general messages, Warn for warnings, Error for error messages)."}</li>
                    <li>{"Keep toast messages concise and informative."}</li>
                    <li>{"Use toasts sparingly to avoid overwhelming the user."}</li>
                    <li>{"Consider the placement of toasts in your UI to ensure they don't obstruct important content."}</li>
                    <li>{"Implement custom styling to match your application's design language."}</li>
                    <li>{"Take advantage of the pause-on-hover feature for important messages that users might need more time to read."}</li>
                </ul>

                <Paragraph>{"The Toast system in Zirv UI provides a powerful and flexible way to display notifications in your Yew application, enhancing user experience and providing important feedback to users."}</Paragraph>
            </Container>
        </section>
    }
}