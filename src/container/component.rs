use yew::{classes, function_component, html, Classes, Html, Properties};
use yew::Children;

#[derive(Properties, Clone, PartialEq)]
pub struct ContainerProps {
    pub classes: Option<Classes>,
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let classes = classes!("style", Some(props.classes.clone()));
    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}