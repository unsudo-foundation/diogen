use super::*;

#[component]
pub fn Stack(props: CommonProps) -> Element {
    rsx!(
        Col {
            class: props.class,
            style: format!(
                r#"
                    position: relative;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}