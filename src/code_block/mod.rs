use yew::{
    function_component,
    html,
    AttrValue,
    Callback,
    Html,
    Properties,
};

pub mod highlight;

use crate::{
    spacing::Padding,
    use_toast,
    Button,
    Toast,
    ToastType,
};

#[derive(Clone, PartialEq, Properties)]
pub struct CodeBlockProps {
    pub snippet: String,
    pub language: String,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let toasts_manager = use_toast::<Toast>();

    let highlighted_html = highlight::highlight_code(&props.snippet, &props.language);

    let onclick = {
        let code = props.snippet.clone();
        Callback::from(move |_| {
            let code = code.clone();

            let window = web_sys::window().expect("window");
            let nav = window.navigator().clipboard();

            match nav {
                Some(a) => {
                    let _ = a.write_text(code.as_str());
                    toasts_manager.spawn(Toast::new(
                        ToastType::Info,
                        "Clipboard",
                        "Code successfully copied to clipboard",
                    ));
                }
                None => {
                    toasts_manager.spawn(Toast::new(ToastType::Error, "Clipboard", "Unable to copy code to clipboard"));
                }
            };
        })
    };

    html! {
        <div class="code-block">
            <div class="code-block_header">
                <span>{&props.language}</span>
                <Button onclick={onclick} padding={Padding::Small}>{"Copy"}</Button>
            </div>
            <div class="code-block_snippet">
                <pre>
                    <code>
                        {Html::from_html_unchecked(AttrValue::from(highlighted_html))}
                    </code>
                </pre>
            </div>
        </div>
    }
}
