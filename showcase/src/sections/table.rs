use yew::{function_component, html, Html};
use zirv_ui::{
    options::flex::{FlexAlign, FlexDirection, FlexGap},
    Container, Headline, Paragraph, CodeBlock, Subheadline, Table,
    options::size::Width,
};

#[function_component(TableSection)]
pub fn table_section() -> Html {
    let headers = vec!["Header 1".to_string(), "Header 2".to_string(), "Header 3".to_string()];
    let data = vec![
        vec!["Row 1, Cell 1".to_string(), "Row 1, Cell 2".to_string(), "Row 1, Cell 3".to_string()],
        vec!["Row 2, Cell 1".to_string(), "Row 2, Cell 2".to_string(), "Row 2, Cell 3".to_string()],
    ];

    html! {
        <section>
            <Container flex_direction={FlexDirection::Column} flex_align={FlexAlign::Start} flex_gap={FlexGap::Large}>
                <Headline>{"Table Component"}</Headline>

                <Paragraph>{"The Table component is a highly customizable table for use in Yew applications using Zirv UI. It supports various layout and styling options for the table, rows, and cells."}</Paragraph>

                <Subheadline>{"Basic Usage"}</Subheadline>
                <Paragraph>{"Here's a simple example of how to use the Table component:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
use yew::{function_component, html, Html};
use zirv_ui::{Table, options::size::Width};

#[function_component(App)]
fn app() -> Html {
    let headers = vec!["Header 1".to_string(), "Header 2".to_string(), "Header 3".to_string()];
    let data = vec![
        vec!["Row 1, Cell 1".to_string(), "Row 1, Cell 2".to_string(), "Row 1, Cell 3".to_string()],
        vec!["Row 2, Cell 1".to_string(), "Row 2, Cell 2".to_string(), "Row 2, Cell 3".to_string()],
    ];

    html! {
        <Table headers={headers} data={data} table_width={Width::Full} />
    }
}
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Properties"}</Subheadline>
                <Paragraph>{"The Table component accepts numerous properties for customization:"}</Paragraph>
                <ul>
                    <li>{"headers: Vec<String> - The headers for the table."}</li>
                    <li>{"data: Vec<Vec<String>> - The data for the table."}</li>
                    <li>{"table_border_width: BorderWidth - The border width of the table."}</li>
                    <li>{"table_border_radius: BorderRadius - The border radius of the table."}</li>
                    <li>{"table_border_style: BorderStyle - The border style of the table."}</li>
                    <li>{"table_width: Width - The width of the table. Default is Width::Max."}</li>
                    <li>{"table_min_height, table_max_height, table_height: Height - Height properties for the table."}</li>
                    <li>{"row_border_width, row_border_radius, row_border_style: Border properties for rows."}</li>
                    <li>{"row_width, row_min_height, row_max_height, row_height: Size properties for rows."}</li>
                    <li>{"cell_border_width, cell_border_radius, cell_border_style: Border properties for cells."}</li>
                    <li>{"cell_width, cell_min_height, cell_max_height, cell_height: Size properties for cells."}</li>
                    <li>{"cell_padding, cell_margin: Height - Spacing properties for cells."}</li>
                    <li>{"cell_text_align: TextAlign - Text alignment for cells."}</li>
                    <li>{"cell_overflow: Overflow - Overflow property for cells. Default is Overflow::Auto."}</li>
                </ul>

                <Subheadline>{"Customization Examples"}</Subheadline>
                
                <Paragraph>{"1. Customizing Table Width and Border:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Table 
    headers={headers} 
    data={data} 
    table_width={Width::Full}
    table_border_width={BorderWidth::Medium}
    table_border_style={BorderStyle::Solid}
/>
                    "#}
                    language="Rust"
                />

                <Paragraph>{"2. Customizing Cell Properties:"}</Paragraph>
                <CodeBlock 
                    snippet={r#"
<Table 
    headers={headers} 
    data={data} 
    cell_padding={Height::Small}
    cell_text_align={TextAlign::Center}
    cell_overflow={Overflow::Hidden}
/>
                    "#}
                    language="Rust"
                />

                <Subheadline>{"Live Example"}</Subheadline>
                <Paragraph>{"Here's a live example of the Table component:"}</Paragraph>

                <Table headers={headers} data={data} table_width={Width::Full} />

                <Subheadline>{"Implementation Details"}</Subheadline>
                <Paragraph>{"The Table component uses Yew's classes macro to apply styling based on the provided properties. It generates the table structure using HTML table elements (<table>, <thead>, <tbody>, <tr>, <th>, <td>) and applies the appropriate classes to each element."}</Paragraph>

                <Subheadline>{"Best Practices"}</Subheadline>
                <Paragraph>{"When using the Table component, consider the following best practices:"}</Paragraph>
                <ul>
                    <li>{"Ensure that the number of items in each data row matches the number of headers."}</li>
                    <li>{"Use appropriate width settings to ensure the table fits well within its container."}</li>
                    <li>{"Consider using cell_overflow when dealing with potentially long content in cells."}</li>
                    <li>{"Use consistent styling across your tables for a uniform look in your application."}</li>
                    <li>{"For large datasets, consider implementing pagination or virtualization to improve performance."}</li>
                </ul>

                <Paragraph>{"The Table component provides a flexible way to display tabular data in your Zirv UI application, with extensive customization options to fit various design needs."}</Paragraph>
            </Container>
        </section>
    }
}