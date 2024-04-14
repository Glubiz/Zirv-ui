use yew::{classes, function_component, html, Classes, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    pub src: String,
    pub alt: String,
    pub classes: Option<Classes>,
}

#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    html! {
        <img 
            src={props.src} 
            alt={props.alt} 
            class={classes!("image", Some(props.classes.clone()))} 
        />
    }
}