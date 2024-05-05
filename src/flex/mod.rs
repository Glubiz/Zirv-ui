use yew::{classes, function_component, html, Children, Classes, Html, Properties};

use crate::options::{
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
pub struct Options {
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

#[derive(Properties, Clone, PartialEq)]
pub struct FlexProps {
    pub children: Children,
    pub options: Options
}

#[function_component(Flex)]
pub fn flex(props: &FlexProps) -> Html {
    html! {
        <div class={classes!("flex", &props.options.direction, &props.options.align, &props.options.justify, Some(props.options.classes.clone()))}>
            {props.children.clone()}
        </div>
    }
}
