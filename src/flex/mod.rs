use yew::{classes, function_component, html, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexDirection {
    #[default]
    Vertical,
    Horizontal,
}

impl From<&FlexDirection> for Classes {
    fn from(direction: &FlexDirection) -> Self {
        match direction {
            FlexDirection::Horizontal => classes!("flex-direction-horizontal"),
            FlexDirection::Vertical => classes!("flex-direction-vertical"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexAlign {
    #[default] 
    Center,
    Start,
    End,
    Baseline,
    Stretch,
}

impl From<&FlexAlign> for Classes {
    fn from(align: &FlexAlign) -> Self {
        match align {
            FlexAlign::Center => classes!("flex-align-center"),
            FlexAlign::Start => classes!("flex-align-start"),
            FlexAlign::End => classes!("flex-align-end"),
            FlexAlign::Baseline => classes!("flex-align-baseline"),
            FlexAlign::Stretch => classes!("flex-align-stretch"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexJustify {
    #[default] 
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Start,
    End,
}

impl From<&FlexJustify> for Classes {
    fn from(justify: &FlexJustify) -> Self {
        match justify {
            FlexJustify::Center => classes!("flex-justify-center"),
            FlexJustify::SpaceBetween => classes!("flex-justify-space-between"),
            FlexJustify::SpaceAround => classes!("flex-justify-space-around"),
            FlexJustify::SpaceEvenly => classes!("flex-justify-space-evenly"),
            FlexJustify::Start => classes!("flex-justify-start"),
            FlexJustify::End => classes!("flex-justify-end"),
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct FlexProps {
    pub children: Children,
    #[prop_or_default]
    pub direction: FlexDirection,
    #[prop_or_default]
    pub align: FlexAlign,
    #[prop_or_default]
    pub justify: FlexJustify,
    pub classes: Option<Classes>,
}

#[function_component(Flex)]
pub fn flex(props: &FlexProps) -> Html {
    html! {
        <div class={classes!("flex", &props.direction, &props.align, &props.justify, Some(props.classes.clone()))}>
            {props.children.clone()}
        </div>
    }
}