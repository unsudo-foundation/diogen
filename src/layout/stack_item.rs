use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct StackItemProps {
    pub z: u64,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn StackItem(props: StackItemProps) -> Element {
    rsx!(
        Col {
            class: props.class,
            style: format!(
                r#"
                    position: absolute;
                    z-index: {};
                    {}
                "#,
                props.z,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}