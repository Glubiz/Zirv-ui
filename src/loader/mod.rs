use yew::{classes, function_component, html, Children, Classes, Html, Properties};

use crate::options::{
    display::Display,
    flex::{
        FlexAlign,
        FlexDirection,
        FlexJustify,
        FlexWrap,
        FlexGrow,
        FlexShrink
    },
    size::{
        Height,
        Width
    }
};

#[derive(Properties, Clone, PartialEq)]
pub struct LoaderProps {
    pub children: Children,
    #[prop_or_default]
    pub wrap: FlexWrap,
    #[prop_or_default]
    pub align: FlexAlign,
    #[prop_or_default]
    pub justify: FlexJustify,
    #[prop_or_default]
    pub frow: FlexGrow,
    #[prop_or_default]
    pub shrink: FlexShrink,
    #[prop_or_default]
    pub direction: FlexDirection,
    #[prop_or_default]
    pub height: Height,
    #[prop_or_default]
    pub width: Width,
    pub classes: Option<Classes>
}

#[function_component(Loader)]
pub fn loader(props: &LoaderProps) -> Html {
    html! {
        <div class={classes!(&Display::Loader, &props.direction, &props.align, &props.justify, Some(props.classes.clone()))}>
            {props.children.clone()}
        </div>
    }
}
