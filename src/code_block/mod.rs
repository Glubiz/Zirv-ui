//! Code Block Component Module
//!
//! This module provides a `CodeBlock` component for rendering and interacting with code snippets.
//! It includes functionality for syntax highlighting and copying code to the clipboard.

use yew::{
    classes, function_component, html, AttrValue, Callback, Html, Properties
};

pub mod highlight;

use crate::{
    font::FontWeight, size::{CustomType, Height, Width}, spacing::Padding, use_toast, Button, Toast, ToastType
};

/// Properties for the CodeBlock component
#[derive(Clone, PartialEq, Properties)]
pub struct CodeBlockProps {
    /// The code snippet to be displayed
    pub snippet: String,
    /// The programming language of the code snippet (for syntax highlighting)
    pub language: String,
}

/// CodeBlock Component
///
/// Renders a code block with syntax highlighting and a copy button.
///
/// # Example
///
/// ```
/// use your_crate::CodeBlock;
///
/// html! {
///     <CodeBlock
///         snippet="fn main() { println!(\"Hello, World!\"); }"
///         language="rust"
///     />
/// }
/// ```
#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    // Initialize the toast manager for displaying notifications
    let toasts_manager = use_toast::<Toast>();

    // Apply syntax highlighting to the code snippet
    let highlighted_html = highlight::highlight_code(&props.snippet, &props.language);

    // Callback for the copy button
    let onclick = {
        let code = props.snippet.clone();
        Callback::from(move |_| {
            let code = code.clone();

            let window = web_sys::window().expect("window");
            let nav = window.navigator().clipboard();

            match nav {
                Some(a) => {
                    // Attempt to copy the code to the clipboard
                    let _ = a.write_text(code.as_str());
                    toasts_manager.spawn(Toast::new(
                        ToastType::Info,
                        "Clipboard",
                        "Code successfully copied to clipboard",
                    ));
                }
                None => {
                    // Display an error if clipboard access is not available
                    toasts_manager.spawn(Toast::new(ToastType::Error, "Clipboard", "Unable to copy code to clipboard"));
                }
            };
        })
    };

    // Render the CodeBlock component
    html! {
        <div class="code-block">
            <div class="code-block_header">
                <span class={classes!(&FontWeight::Bold)}>{&props.language}</span>
                <Button onclick={onclick} padding={Padding::Small} width={Width::Custom(8, CustomType::Fixed)} height={Height::Custom(3, CustomType::Fixed)}>{"Copy"}</Button>
            </div>
            <div class="code-block_snippet">
                <pre>
                    <code>
                        // Render the highlighted code as HTML
                        {Html::from_html_unchecked(AttrValue::from(highlighted_html))}
                    </code>
                </pre>
            </div>
        </div>
    }
}