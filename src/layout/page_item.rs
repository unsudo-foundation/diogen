use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PageItemProps {
    pub background: Option<Element>,
    pub top: Option<Element>,
    pub bottom: Option<Element>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn PageItem(props: PageItemProps) -> Element {
    rsx!(
        Stack {
            class: props.class,
            style: format!(
                r#"
                    flex: 1;
                    {}
                    {}
                "#,
                FILL_VIEW,
                props.style.unwrap_or_default()
            ),
            StackItem {
                z: 0,
                style: format!(
                    r#"
                        {}
                        {}
                    "#,
                    ABS_POS_RESET,
                    FILL_VIEW
                ),
                { props.background }
            }
            StackItem {
                z: 1,
                style: format!(
                    r#"
                        justify-content: space-between;
                        scroll-snap-align: start;
                        {}
                        {}
                    "#,
                    ABS_POS_RESET,
                    FILL_VIEW
                ),
                { props.top }
                { props.children }
                { props.bottom }
            }
        }
    )
}