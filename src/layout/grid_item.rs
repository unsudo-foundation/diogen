use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct GridItemProps {
    pub x_from: u8,
    pub y_from: u8,
    pub x_to: u8,
    pub y_to: u8,
    pub z: Option<u128>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn GridItem(props: GridItemProps) -> Element {
    rsx!(
        div {
            class: props.class,
            style: format!(
                r#"
                    grid-column: {} / {};
                    grid-row: {} / {};
                    z-index: {};
                    {}
                "#,
                props.x_from,
                props.x_to,
                props.y_from,
                props.y_to,
                props.z.unwrap_or_default(),
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}