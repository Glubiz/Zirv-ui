use yew::{classes, function_component, html, Children, Classes, Html, Properties};

use crate::options::font::{
    FontSize,
    FontWeight,
    FontStyle,
    FontFamily,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Options {
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

#[derive(Properties, Clone, PartialEq)]
pub struct SubheadlineProps {
    pub children: Children,
    pub options: Options
}

#[function_component(Subheadline)]
pub fn subheadline(props: &SubheadlineProps) -> Html {
    html! {
        <h3 class={classes!(&props.options.size, &props.options.weight, &props.options.style, &props.options.family, &props.options.classes)}>
            {props.children.clone()}
        </h3>
    }
}