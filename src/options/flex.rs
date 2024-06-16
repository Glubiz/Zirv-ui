//! Flexbox Enums
//! 
//! This module defines several enums representing different flexbox properties: `FlexDirection`, `FlexWrap`,
//! `FlexAlign`, `FlexJustify`, `FlexGrow`, `FlexShrink`, and `Flex`. Each enum can be converted into Yew's `Classes`
//! for CSS styling.
//! 
//! # Example
//! 
//! ```rust
//! use yew::{html, function_component, Html};
//! use crate::options::flex::{Flex, FlexAlign, FlexDirection, FlexJustify, FlexWrap};
//! use yew::Classes;
//! 
//! #[function_component(App)]
//! fn app() -> Html {
//!     let flex_class: Classes = (&Flex::Row(FlexAlign::Center, FlexJustify::SpaceBetween, FlexWrap::Wrap)).into();
//! 
//!     html! {
//!         <div class={flex_class}>
//!             {"This div has a flex display with specified properties"}
//!         </div>
//!     }
//! }
//! ```

use yew::{classes, Classes};

/// Enum representing the flex direction options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexDirection {
    /// Flex direction row.
    #[default]
    Row,
    /// Flex direction row-reverse.
    RowReverse,
    /// Flex direction column.
    Column,
    /// Flex direction column-reverse.
    ColumnReverse,
}

impl From<&FlexDirection> for Classes {
    /// Converts a `FlexDirection` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::flex::FlexDirection;
    /// 
    /// let direction_class: Classes = (&FlexDirection::Row).into();
    /// ```
    fn from(flex_direction: &FlexDirection) -> Self {
        match flex_direction {
            FlexDirection::Row => classes!("flex-direction-row"),
            FlexDirection::RowReverse => classes!("flex-direction-row-reverse"),
            FlexDirection::Column => classes!("flex-direction-column"),
            FlexDirection::ColumnReverse => classes!("flex-direction-column-reverse"),
        }
    }
}

/// Enum representing the flex wrap options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexWrap {
    /// No wrap.
    #[default]
    NoWrap,
    /// Wrap.
    Wrap,
    /// Wrap reverse.
    WrapReverse,
}

impl From<&FlexWrap> for Classes {
    /// Converts a `FlexWrap` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::flex::FlexWrap;
    /// 
    /// let wrap_class: Classes = (&FlexWrap::Wrap).into();
    /// ```
    fn from(flex_wrap: &FlexWrap) -> Self {
        match flex_wrap {
            FlexWrap::NoWrap => classes!("flex-wrap-nowrap"),
            FlexWrap::Wrap => classes!("flex-wrap-wrap"),
            FlexWrap::WrapReverse => classes!("flex-wrap-wrap-reverse"),
        }
    }
}

/// Enum representing the flex alignment options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexAlign {
    /// Align items at the start.
    Start,
    /// Align items at the end.
    End,
    /// Align items at the center. This is the default.
    #[default]
    Center,
    /// Align items at the baseline.
    Baseline,
    /// Stretch items to fill the container.
    Stretch,
}

impl From<&FlexAlign> for Classes {
    /// Converts a `FlexAlign` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::flex::FlexAlign;
    /// 
    /// let align_class: Classes = (&FlexAlign::Center).into();
    /// ```
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

/// Enum representing the flex justify options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexJustify {
    /// Justify items at the start.
    Start,
    /// Justify items at the end.
    End,
    /// Justify items at the center.
    Center,
    /// Justify items with space between them. This is the default.
    #[default]
    SpaceBetween,
    /// Justify items with space around them.
    SpaceAround,
    /// Justify items with space evenly distributed.
    SpaceEvenly,
}

impl From<&FlexJustify> for Classes {
    /// Converts a `FlexJustify` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::flex::FlexJustify;
    /// 
    /// let justify_class: Classes = (&FlexJustify::SpaceBetween).into();
    /// ```
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

/// Enum representing the flex grow options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexGrow {
    /// No flex grow. This is the default.
    #[default]
    None,
    /// Flex grow.
    Grow,
}

impl From<&FlexGrow> for Classes {
    /// Converts a `FlexGrow` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::flex::FlexGrow;
    /// 
    /// let grow_class: Classes = (&FlexGrow::Grow).into();
    /// ```
    fn from(flex_grow: &FlexGrow) -> Self {
        match flex_grow {
            FlexGrow::None => classes!("flex-grow-0"),
            FlexGrow::Grow => classes!("flex-grow-1"),
        }
    }
}

/// Enum representing the flex shrink options.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FlexShrink {
    /// No flex shrink. This is the default.
    #[default]
    None,
    /// Flex shrink.
    Shrink,
}

impl From<&FlexShrink> for Classes {
    /// Converts a `FlexShrink` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::flex::FlexShrink;
    /// 
    /// let shrink_class: Classes = (&FlexShrink::Shrink).into();
    /// ```
    fn from(flex_shrink: &FlexShrink) -> Self {
        match flex_shrink {
            FlexShrink::None => classes!("flex-shrink-0"),
            FlexShrink::Shrink => classes!("flex-shrink-1"),
        }
    }
}

/// Enum representing the combined flex properties.
#[derive(Debug, Clone, PartialEq)]
pub enum Flex {
    /// No flex properties.
    None,
    /// Flex row with specified alignment, justification, and wrapping.
    Row(FlexAlign, FlexJustify, FlexWrap),
    /// Flex column with specified alignment, justification, and wrapping.
    Column(FlexAlign, FlexJustify, FlexWrap),
}

impl From<&Flex> for Classes {
    /// Converts a `Flex` into Yew's `Classes`.
    ///
    /// # Example
    /// 
    /// ```rust
    /// use yew::Classes;
    /// use crate::options::flex::{Flex, FlexAlign, FlexJustify, FlexWrap};
    /// 
    /// let flex_class: Classes = (&Flex::Row(FlexAlign::Center, FlexJustify::SpaceBetween, FlexWrap::Wrap)).into();
    /// ```
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
