use syntect::{
    highlighting::ThemeSet,
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};
use wasm_bindgen::JsCast;
use yew::{
    classes,
    function_component,
    html,
    AttrValue,
    Callback,
    Html,
    Properties,
};

use crate::{
    border::BorderRadius,
    color::BackgroundColor,
    font::TextColor,
    spacing::Padding,
};

#[derive(Clone, PartialEq, Properties)]
pub struct CodeBlockProps {
    pub code: String,
    pub language: String,
    #[prop_or_default]
    pub padding: Padding,
    #[prop_or(BackgroundColor::PrimaryDark)]
    pub background_color: BackgroundColor,
    #[prop_or_default]
    pub text_color: TextColor,
    #[prop_or_default]
    pub border_radius: BorderRadius,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let classes = classes!(&props.padding, &props.background_color, &props.text_color, &props.border_radius,);

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    let syntax =
        syntax_set.find_syntax_by_extension(&props.language).unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let theme = &theme_set.themes["base16-ocean.dark"];

    let highlighted_html =
        highlighted_html_for_string(&props.code, &syntax_set, syntax, theme).unwrap_or_else(|_| props.code.clone());

    let onclick = {
        let code = props.code.clone();
        Callback::from(move |_| {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            // Create a temporary textarea element
            let temp = document.create_element("textarea").unwrap();
            let temp = temp.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();

            // Set its value to the code and append it to the body
            temp.set_value(&code);
            document.body().unwrap().append_child(&temp).unwrap();

            // Select the text
            temp.select();

            // Inform the user that the text has been copied
            window.alert_with_message("Code copied to clipboard. Please use Ctrl+V or Cmd+V to paste.").unwrap();

            // Remove the temporary element
            document.body().unwrap().remove_child(&temp).unwrap();
        })
    };

    html! {
        <div class={classes}>
            <pre>
                <code class={format!("language-{}", props.language)}>
                    {Html::from_html_unchecked(AttrValue::from(highlighted_html))}
                </code>
            </pre>
            <button onclick={onclick} class="copy-button">{"Copy"}</button>
        </div>
    }
}
