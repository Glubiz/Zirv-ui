use yew::{classes, function_component, html, Children, Classes, Html, Properties};

#[derive(Properties, Clone, PartialEq, Default)]
pub struct ContainerProps {
    pub children: Children,
    pub classes: Option<Classes>,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    html! {
        <div class={classes!("container", Some(props.classes.clone()))}>
            { props.children.clone() }
        </div>
    }
}