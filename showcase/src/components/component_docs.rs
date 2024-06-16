use yew::{function_component, html, Html, Properties};
use zirv_ui::{Headline, Paragraph};

#[derive(Clone, PartialEq, Properties)]
pub struct ComponentDocsProps {
    pub component_name: String,
    pub description: String,
    pub props: Vec<String>,
}

#[function_component(ComponentDocs)]
pub fn component_docs(props: &ComponentDocsProps) -> Html {
    html! {
        <div>
            <Headline>{&*props.component_name}</Headline>
            <Paragraph>{&*props.description}</Paragraph>
        </div>
    }
}
