use yew::{classes, function_component, html, Html, Properties};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Style {
    Dots,
    #[default]
    Spinner,
}

#[derive(Properties, Clone, PartialEq)]
pub struct LoaderProps {
    #[prop_or_default]
    pub style: Style,
}

#[function_component(Loader)]
pub fn loader(props: &LoaderProps) -> Html {
    match props.style {
        Style::Dots => html! {
            <div class={classes!("loader-dots")}>
                <div class={classes!("dot")}></div>
                <div class={classes!("dot")}></div>
                <div class={classes!("dot")}></div>
            </div>
        },
        Style::Spinner => html! {
            <div class={classes!("loader-spinner")}></div>
        },
    }
}
