use yew::{classes, function_component, html, Html, Properties};

use crate::options::{
    border::{BorderRadius, BorderStyle, BorderWidth},
    font::TextAlign,
    overflow::Overflow,
    size::{Height, Width},
};

#[derive(Clone, PartialEq, Properties, Default)]
pub struct TableProps {
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
    #[prop_or_default]
    pub table_border_width: BorderWidth,
    #[prop_or_default]
    pub table_border_radius: BorderRadius,
    #[prop_or_default]
    pub table_border_style: BorderStyle,
    #[prop_or(Width::Max)]
    pub table_width: Width,
    #[prop_or_default]
    pub table_min_height: Height,
    #[prop_or_default]
    pub table_max_height: Height,
    #[prop_or_default]
    pub table_height: Height,
    #[prop_or_default]
    pub row_border_width: BorderWidth,
    #[prop_or_default]
    pub row_border_radius: BorderRadius,
    #[prop_or_default]
    pub row_border_style: BorderStyle,
    #[prop_or_default]
    pub row_width: Width,
    #[prop_or_default]
    pub row_min_height: Height,
    #[prop_or_default]
    pub row_max_height: Height,
    #[prop_or_default]
    pub row_height: Height,
    #[prop_or_default]
    pub cell_border_width: BorderWidth,
    #[prop_or_default]
    pub cell_border_radius: BorderRadius,
    #[prop_or_default]
    pub cell_border_style: BorderStyle,
    #[prop_or_default]
    pub cell_width: Width,
    #[prop_or_default]
    pub cell_min_height: Height,
    #[prop_or_default]
    pub cell_max_height: Height,
    #[prop_or_default]
    pub cell_height: Height,
    #[prop_or_default]
    pub cell_padding: Height,
    #[prop_or_default]
    pub cell_margin: Height,
    #[prop_or_default]
    pub cell_text_align: TextAlign,
    #[prop_or(Overflow::Auto)]
    pub cell_overflow: Overflow,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let table_classes = classes! {
        &props.table_border_width,
        &props.table_border_radius,
        &props.table_border_style,
        &props.table_width,
        &props.table_min_height,
        &props.table_max_height,
        &props.table_height,
    };

    let row_classes = classes! {
        &props.row_border_width,
        &props.row_border_radius,
        &props.row_border_style,
        &props.row_width,
        &props.row_min_height,
        &props.row_max_height,
        &props.row_height,
    };

    let cell_classes = classes! {
        &props.cell_border_width,
        &props.cell_border_radius,
        &props.cell_border_style,
        &props.cell_width,
        &props.cell_min_height,
        &props.cell_max_height,
        &props.cell_height,
        &props.cell_padding,
        &props.cell_margin,
        &props.cell_text_align,
    };

    let headers = props
        .headers
        .iter()
        .map(|header| html! { <th>{header}</th> })
        .collect::<Html>();

    let data = props.data.iter().map(|row| {
        let row = row
            .iter()
            .map(|cell| html! { <td class={cell_classes}>{cell}</td> })
            .collect::<Html>();
        html! { <tr class={row_classes}>{row}</tr> }
    });

    html! {
        <table class={table_classes}>
            <tr>
                {headers}
            </tr>
            {data}
        </table>
    }
}
