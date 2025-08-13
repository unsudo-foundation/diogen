use super::*;

#[component]
pub fn Row(props: CommonProps) -> Element {
    rsx!(
        Col {
            class: props.class,
            style: format!(
                r#"
                    flex-direction: row;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}