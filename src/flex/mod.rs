use yew::{classes, function_component, html, Children, Classes, Html, Properties};

pub struct Options {
    pub direction: Option<Direction>,
    pub align: Option<Align>,
    pub justify: Option<Justify>,
    pub classes: Option<Classes>
}

#[derive(Properties, Clone, PartialEq)]
pub struct FlexProps {
    pub children: Children,
    pub options: Options
}

#[function_component(Flex)]
pub fn flex(props: &FlexProps) -> Html {
    html! {
        <div class={classes!("flex", &props.direction, &props.align, &props.justify, Some(props.classes.clone()))}>
            {props.children.clone()}
        </div>
    }
}
