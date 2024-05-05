use yew::{classes, Classes};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl From<&FlexDirection> for Classes {
    fn from(flex_direction: &FlexDirection) -> Self {
        match flex_direction {
            FlexDirection::Row => classes!("flex-direction-row"),
            FlexDirection::RowReverse => classes!("flex-direction-row-reverse"),
            FlexDirection::Column => classes!("flex-direction-column"),
            FlexDirection::ColumnReverse => classes!("flex-direction-column-reverse"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

impl From<&FlexWrap> for Classes {
    fn from(flex_wrap: &FlexWrap) -> Self {
        match flex_wrap {
            FlexWrap::NoWrap => classes!("flex-wrap-nowrap"),
            FlexWrap::Wrap => classes!("flex-wrap-wrap"),
            FlexWrap::WrapReverse => classes!("flex-wrap-wrap-reverse"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexAlign {
    Start,
    End,
    #[default]
    Center,
    Baseline,
    Stretch,
}

impl From<&FlexAlign> for Classes {
    fn from(flex_align: &FlexAlign) -> Self {
        match flex_align {
            FlexAlign::Start => classes!("flex-align-start"),
            FlexAlign::End => classes!("flex-align-end"),
            FlexAlign::Center => classes!("flex-align-center"),
            FlexAlign::Baseline => classes!("flex-align-baseline"),
            FlexAlign::Stretch => classes!("flex-align-stretch"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexJustify {
    Start,
    End,
    Center,
    #[default]
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl From<&FlexJustify> for Classes {
    fn from(flex_justify: &FlexJustify) -> Self {
        match flex_justify {
            FlexJustify::Start => classes!("flex-justify-start"),
            FlexJustify::End => classes!("flex-justify-end"),
            FlexJustify::Center => classes!("flex-justify-center"),
            FlexJustify::SpaceBetween => classes!("flex-justify-space-between"),
            FlexJustify::SpaceAround => classes!("flex-justify-space-around"),
            FlexJustify::SpaceEvenly => classes!("flex-justify-space-evenly"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexGrow {
    #[default]
    None,
    Grow,
}

impl From<&FlexGrow> for Classes {
    fn from(flex_grow: &FlexGrow) -> Self {
        match flex_grow {
            FlexGrow::None => classes!("flex-grow-0"),
            FlexGrow::Grow => classes!("flex-grow-1"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexShrink {
    #[default]
    None,
    Shrink,
}

impl From<&FlexShrink> for Classes {
    fn from(flex_shrink: &FlexShrink) -> Self {
        match flex_shrink {
            FlexShrink::None => classes!("flex-shrink-0"),
            FlexShrink::Shrink => classes!("flex-shrink-1"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Flex {
    None,
    Row(FlexAlign, FlexJustify, FlexWrap),
    Column(FlexAlign, FlexJustify, FlexWrap),
}

impl From<&Flex> for Classes {
    fn from(flex: &Flex) -> Self {
        match flex {
            Flex::None => classes!("flex-none"),
            Flex::Row(align, justify, wrap) => {
                let mut classes = classes!("flex-row");
                classes.push(align);
                classes.push(justify);
                classes.push(wrap);
                classes
            }
            Flex::Column(align, justify, wrap) => {
                let mut classes = classes!("flex-column");
                classes.push(align);
                classes.push(justify);
                classes.push(wrap);
                classes
            }
        }
    }
}
