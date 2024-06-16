use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq, Default)]
pub enum ListStyle {
    Ordered,
    #[default]
    Unordered,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListProps<T: Into<String>> {
    pub data: Vec<T>,
    #[prop_or_default]
    pub style: Option<ListStyle>,
}

#[function_component(List<T>)]
pub fn list<T>(props: &ListProps<T>) -> Html {
    let classes = match &props.style {
        Some(style) => match style {
            ListStyle::Ordered => "list-decimal",
            ListStyle::Unordered => "list-disc",
        },
        None => "list-disc",
    };

    match &props.style {
        Some(style) => {
            html! {
                <ol class={classes}>
                    {"List"}
                </ol>
            }
        }
        None => {
            html! {
                <ul class={classes}>
                    {"List"}
                </ul>
            }
        }
    }
}
