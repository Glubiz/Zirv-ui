//! Table Component
//!
//! This module provides a customizable table component for the Yew framework. The `Table` component
//! supports various layout and styling options for the table, rows, and cells, such as border
//! properties, width, height, padding, margin, text alignment, and overflow.
//!
//! # Example
//!
//! ```rust
//! use yew::{
//!     function_component,
//!     html,
//!     Html,
//! };
//! use zirv_ui::{
//!     options::size::Width,
//!     Table,
//!     TableProps,
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let headers = vec!["Header 1".to_string(), "Header 2".to_string(), "Header 3".to_string()];
//!     let data = vec![
//!         vec![
//!             "Row 1, Cell 1".to_string(),
//!             "Row 1, Cell 2".to_string(),
//!             "Row 1, Cell 3".to_string(),
//!         ],
//!         vec![
//!             "Row 2, Cell 1".to_string(),
//!             "Row 2, Cell 2".to_string(),
//!             "Row 2, Cell 3".to_string(),
//!         ],
//!     ];
//!
//!     html! {
//!         <Table headers={headers} data={data} table_width={Width::Full} />
//!     }
//! }
//! ```

use std::cmp::Ordering;

use yew::{
    classes, function_component, html, use_state_eq, Callback, Html, Properties
};

use crate::options::{
    border::{
        BorderRadius,
        BorderStyle,
        BorderWidth,
    },
    font::TextAlign,
    overflow::Overflow,
    size::{
        Height,
        Width,
    },
};

#[derive(Clone, Copy, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    None,
}

/// Properties for the `Table` component.
#[derive(Clone, PartialEq, Properties, Default)]
pub struct TableProps {
    /// The headers for the table.
    pub headers: Vec<String>,
    /// The data for the table.
    pub data: Vec<Vec<String>>,
    /// Callback for when a column is sorted
    #[prop_or_default]
    pub on_sort: Option<Callback<(usize, SortDirection)>>,
    /// The border width of the table.
    #[prop_or_default]
    pub table_border_width: BorderWidth,
    /// The border radius of the table.
    #[prop_or_default]
    pub table_border_radius: BorderRadius,
    /// The border style of the table.
    #[prop_or_default]
    pub table_border_style: BorderStyle,
    /// The width of the table. Default is `Width::Max`.
    #[prop_or(Width::Max)]
    pub table_width: Width,
    /// The minimum height of the table.
    #[prop_or_default]
    pub table_min_height: Height,
    /// The maximum height of the table.
    #[prop_or_default]
    pub table_max_height: Height,
    /// The height of the table.
    #[prop_or_default]
    pub table_height: Height,
    /// The border width of the rows.
    #[prop_or_default]
    pub row_border_width: BorderWidth,
    /// The border radius of the rows.
    #[prop_or_default]
    pub row_border_radius: BorderRadius,
    /// The border style of the rows.
    #[prop_or_default]
    pub row_border_style: BorderStyle,
    /// The width of the rows.
    #[prop_or_default]
    pub row_width: Width,
    /// The minimum height of the rows.
    #[prop_or_default]
    pub row_min_height: Height,
    /// The maximum height of the rows.
    #[prop_or_default]
    pub row_max_height: Height,
    /// The height of the rows.
    #[prop_or_default]
    pub row_height: Height,
    /// The border width of the cells.
    #[prop_or_default]
    pub cell_border_width: BorderWidth,
    /// The border radius of the cells.
    #[prop_or_default]
    pub cell_border_radius: BorderRadius,
    /// The border style of the cells.
    #[prop_or_default]
    pub cell_border_style: BorderStyle,
    /// The width of the cells.
    #[prop_or_default]
    pub cell_width: Width,
    /// The minimum height of the cells.
    #[prop_or_default]
    pub cell_min_height: Height,
    /// The maximum height of the cells.
    #[prop_or_default]
    pub cell_max_height: Height,
    /// The height of the cells.
    #[prop_or_default]
    pub cell_height: Height,
    /// The padding of the cells.
    #[prop_or_default]
    pub cell_padding: Height,
    /// The margin of the cells.
    #[prop_or_default]
    pub cell_margin: Height,
    /// The text alignment of the cells.
    #[prop_or_default]
    pub cell_text_align: TextAlign,
    /// The overflow property of the cells. Default is `Overflow::Auto`.
    #[prop_or(Overflow::Auto)]
    pub cell_overflow: Overflow,
}

/// The `Table` component.
///
/// The `Table` component is used to display tabular data with customizable properties for the
/// table, rows, and cells. It supports various styling options such as border properties, width,
/// height, padding, margin, text alignment, and overflow.
///
/// # Properties
///
/// - `headers`: The headers for the table.
/// - `data`: The data for the table.
/// - `table_border_width`: The border width of the table.
/// - `table_border_radius`: The border radius of the table.
/// - `table_border_style`: The border style of the table.
/// - `table_width`: The width of the table. Default is `Width::Max`.
/// - `table_min_height`: The minimum height of the table.
/// - `table_max_height`: The maximum height of the table.
/// - `table_height`: The height of the table.
/// - `row_border_width`: The border width of the rows.
/// - `row_border_radius`: The border radius of the rows.
/// - `row_border_style`: The border style of the rows.
/// - `row_width`: The width of the rows.
/// - `row_min_height`: The minimum height of the rows.
/// - `row_max_height`: The maximum height of the rows.
/// - `row_height`: The height of the rows.
/// - `cell_border_width`: The border width of the cells.
/// - `cell_border_radius`: The border radius of the cells.
/// - `cell_border_style`: The border style of the cells.
/// - `cell_width`: The width of the cells.
/// - `cell_min_height`: The minimum height of the cells.
/// - `cell_max_height`: The maximum height of the cells.
/// - `cell_height`: The height of the cells.
/// - `cell_padding`: The padding of the cells.
/// - `cell_margin`: The margin of the cells.
/// - `cell_text_align`: The text alignment of the cells.
/// - `cell_overflow`: The overflow property of the cells. Default is `Overflow::Auto`.
#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let sort_column = use_state_eq(|| None::<usize>);
    let sort_direction = use_state_eq(|| SortDirection::None);

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

    let headers = props.headers.iter().enumerate().map(|(index, header)| {
        let sort_column = sort_column.clone();
        let sort_direction = sort_direction.clone();
        let on_sort = props.on_sort.clone();

        let sort_indicator = if Some(index) == *sort_column {
            match *sort_direction {
                SortDirection::Ascending => " ▲",
                SortDirection::Descending => " ▼",
                SortDirection::None => "",
            }
        } else {
            ""
        };

        let onclick = Callback::from(move |_| {
            let new_direction = if Some(index) == *sort_column {
                match *sort_direction {
                    SortDirection::Ascending => SortDirection::Descending,
                    SortDirection::Descending => SortDirection::None,
                    SortDirection::None => SortDirection::Ascending,
                }
            } else {
                SortDirection::Ascending
            };

            sort_column.set(Some(index));
            sort_direction.set(new_direction);

            if let Some(callback) = on_sort.as_ref() {
                callback.emit((index, new_direction));
            }
        });

        html! {
            <th onclick={onclick}>
                {header}
                {sort_indicator}
            </th>
        }
    }).collect::<Html>();

    let mut sorted_data = props.data.clone();
    if let Some(column) = *sort_column {
        sorted_data.sort_by(|a, b| {
            let comparison = a[column].cmp(&b[column]);
            match *sort_direction {
                SortDirection::Ascending => comparison,
                SortDirection::Descending => comparison.reverse(),
                SortDirection::None => Ordering::Equal,
            }
        });
    }

    let data = sorted_data
        .iter()
        .map(|row| {
            let cells = row.iter().map(|cell| html! { <td class={cell_classes.clone()}>{cell}</td> }).collect::<Html>();
            html! { <tr class={row_classes.clone()}>{cells}</tr> }
        })
        .collect::<Html>();


    html! {
        <table class={table_classes}>
            <thead>
                <tr>
                    {headers}
                </tr>
            </thead>
            <tbody>
                {data}
            </tbody>
        </table>
    }
}
