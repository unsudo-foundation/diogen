use super::*;

#[component]
pub fn Col(props: CommonProps) -> Element {
    rsx!(
        div {
            class: props.class,
            style: format!(
                r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}