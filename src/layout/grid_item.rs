use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct GridItemProps {
    pub from_x: u8,
    pub from_y: u8,
    pub to_x: u8,
    pub to_y: u8,
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
                props.from_x,
                props.to_x,
                props.from_y,
                props.to_y,
                props.z.unwrap_or_default(),
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}