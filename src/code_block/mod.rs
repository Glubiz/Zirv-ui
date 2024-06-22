use yew::{
    classes,
    function_component,
    html,
    Children,
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
    pub children: Children,
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

    html! {
        <pre class={classes}>
            <code>
                {props.children.clone()}
            </code>
        </pre>
    }
}
