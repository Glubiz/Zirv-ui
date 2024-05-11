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
pub struct HeadlineProps {
    pub children: Children,
    pub options: Options
}

#[function_component(Headline)]
pub fn headline(props: &HeadlineProps) -> Html {
    html! {
        <h1 class={classes!(todo!())}>
            {props.children.clone()}
        </h1>
    }
}
