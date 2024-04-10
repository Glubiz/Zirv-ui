use yew::{classes, function_component, html, Children, Classes, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    pub src: Into<String>,
    pub alt: Option<String>,
    pub classes: Option<Classes>,
}

#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    html! {
        <img 
            src={&props.image} 
            alt={Some(props.alt.clone())} 
            class={classes!("image", Some(props.classes.clone()))} 
        />
    }
}