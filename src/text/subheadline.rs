use yew::{classes, function_component, html, Children, Classes, Html, Properties};

use crate::options::font::{
    FontSize,
    FontWeight,
    FontStyle,
    FontFamily,
};

#[derive(Properties, Clone, PartialEq)]
pub struct SubheadlineProps {
    pub children: Children,
    #[prop_or_default]
    pub size: FontSize,
    #[prop_or_default]
    pub weight: FontWeight,
    #[prop_or_default]
    pub style: FontStyle,
    #[prop_or_default]
    pub family: FontFamily,
    pub classes: Option<Classes>
}

#[function_component(Subheadline)]
pub fn subheadline(props: &SubheadlineProps) -> Html {
    let classes = classes!(&props.size, &props.weight, &props.style, &props.family, &props.classes);

    html! {
        <h3 class={classes!(classes)}>
            {props.children.clone()}
        </h3>
    }
}