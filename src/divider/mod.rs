use yew::{
    classes,
    function_component,
    html,
    Html,
    Properties,
};

use crate::{
    border::BorderRadius,
    color::BackgroundColor,
};

#[derive(Clone, PartialEq, Properties)]
pub struct DividerProps {
    #[prop_or_default]
    pub classes: String,
    #[prop_or(BackgroundColor::Primary)]
    pub color: BackgroundColor,
    #[prop_or_default]
    pub border_radius: BorderRadius,
}

#[function_component(Divider)]
pub fn divider(props: &DividerProps) -> Html {
    let classes = classes!("divider", &props.color, &props.border_radius, &props.classes);

    html! {
        <div class={classes}>
        </div>
    }
}