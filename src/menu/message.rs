use yew::prelude::*;

use crate::options::{
    size::{Height, Width},
    z_index::ZIndex,
};

#[derive(Properties, PartialEq, Default)]
pub struct MenuProps {
    pub children: Children,
    #[prop_or(Width::Custom(12))]
    pub width: Width,
    #[prop_or(Height::Full)]
    pub height: Height,
    #[prop_or(ZIndex::Front)]
    pub z_index: ZIndex,
    #[prop_or(None)]
    pub classes: Option<String>,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let classes = classes!("menu", &props.width, &props.height, &props.z_index, &props.classes);

    html! {
        <div class={classes}>
            { &props.children }
        </div>
    }
}
