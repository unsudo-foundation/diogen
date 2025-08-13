use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct GridProps {
    pub row_count: u8,
    pub row_gap: Option<String>,
    pub col_count: u8,
    pub col_gap: Option<String>,
    pub gap: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    rsx!(
        div {
            style: format!(
                r#"
                    display: grid;
                    grid-template-rows: repeat(1fr, {});
                    grid-template-columns: repeat(1fr, {});
                    gap: {};
                    row-gap: {};
                    column-gap: {};
                    {}
                "#,
                props.row_count,
                props.col_count,
                props.gap.unwrap_or(String::from("0")),
                props.row_gap.unwrap_or(String::from("0")),
                props.col_gap.unwrap_or(String::from("0")),
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}