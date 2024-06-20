use yew::{
    classes,
    function_component,
    html,
    Classes,
    Html,
    Properties,
};

#[derive(Clone, PartialEq, Default)]
pub enum ListStyle {
    Ordered,
    #[default]
    Unordered,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListProps {
    pub data: Vec<String>,
    #[prop_or_default]
    pub style: Option<ListStyle>,
    #[prop_or(None)]
    pub classes: Option<Classes>,
}

#[function_component(List)]
pub fn list<T>(props: &ListProps) -> Html {
    match &props.style {
        Some(style) => match style {
            ListStyle::Ordered => {
                html! {
                    <ol class={classes!(&props.classes)}>
                        {props.data.iter().map(|item| html! { <li>{item}</li> }).collect::<Vec<_>>()}
                    </ol>
                }
            }
            ListStyle::Unordered => {
                html! {
                    <ul class={classes!(&props.classes)}>
                        {props.data.iter().map(|item| html! { <li>{item}</li> }).collect::<Vec<_>>()}
                    </ul>
                }
            }
        },
        None => {
            html! {
                <ul class={classes!(&props.classes)}>
                    {props.data.iter().map(|item| html! { <li>{item}</li> }).collect::<Vec<_>>()}
                </ul>
            }
        }
    }
}
