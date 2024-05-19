use yew::{classes, function_component, html, Children, Classes, Html, Properties};

use crate::options::{
    color::BackgroundColor, border::{Border, BorderColor, BorderRadius, BorderStyle, BorderWidth}, display::Display, flex::{FlexAlign, FlexDirection, FlexGrow, FlexJustify, FlexShrink, FlexWrap}, size::{Height, Width}, spacing::{Margin, Padding}
};

#[derive(Properties, Clone, PartialEq, Default)]
pub struct ContainerProps {
    pub children: Children,
    #[prop_or_default]
    pub width: Width,
    #[prop_or_default]
    pub height: Height,
    #[prop_or_default]
    pub display: Display,
    #[prop_or_default]
    pub flex_direction: FlexDirection,
    #[prop_or_default]
    pub flex_wrap: FlexWrap,
    #[prop_or_default]
    pub flex_grow: FlexGrow,
    #[prop_or_default]
    pub flex_shrink: FlexShrink,
    #[prop_or_default]
    pub flex_align: FlexAlign,
    #[prop_or_default]
    pub flex_justify: FlexJustify,
    #[prop_or_default]
    pub border: Border,
    #[prop_or_default]
    pub border_radius: BorderRadius,
    #[prop_or_default]
    pub border_color: BorderColor,
    #[prop_or_default]
    pub border_width: BorderWidth,
    #[prop_or_default]
    pub border_style: BorderStyle,
    #[prop_or_default]
    pub padding: Padding,
    #[prop_or_default]
    pub margin: Margin,
    #[prop_or(BackgroundColor::Container)]
    pub background_color: BackgroundColor,
    #[prop_or(None)]
    pub classes: Option<Classes>
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let classes = classes!(
        &props.width,
        &props.height,
        &props.display,
        &props.flex_direction,
        &props.flex_wrap,
        &props.flex_grow,
        &props.flex_shrink,
        &props.flex_align,
        &props.flex_justify,
        &props.border,
        &props.border_radius,
        &props.border_color,
        &props.border_width,
        &props.border_style,
        &props.padding,
        &props.margin,
        &props.background_color,
        &props.classes
    );

    html! {
        <div class={classes!(classes)}>
            { props.children.clone() }
        </div>
    }
}
