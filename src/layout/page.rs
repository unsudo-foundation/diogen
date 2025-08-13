use super::*;

pub static PAGE_OVERLAY_Z_INDEX: u64 = u64::MAX / 2;

#[repr(u8)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum PageScrollSnap {
    Mandatory,
    Proximity
}

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PageProps {
    pub scroll_snap: Option<PageScrollSnap>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub overlay: Option<Element>,
    pub children: Option<Element>
}

#[component]
pub fn Page(props: PageProps) -> Element {
    rsx!(
        Stack {
            class: props.class,
            style: format!(
                r#"
                    justify-content: start;
                    {}
                    {}
                "#,
                FILL_VIEW,
                props.style.unwrap_or_default()
            ),
            StackItem {
                z: PAGE_OVERLAY_Z_INDEX,
                style: format!(
                    r#"
                        justify-content: start;
                        overflow-x: hidden;
                        overflow-y: hidden;
                        pointer-events: none;
                        {}
                    "#,
                    FILL
                ),
                { props.overlay }
            }
            StackItem {
                z: 0,
                style: format!(
                    r#"
                        justify-content: start;
                        overflow-x: hidden;
                        overflow-y: auto;
                        scroll-snap-type: {};
                        scroll-behaviour: smooth;
                        {}
                    "#,
                    match props.scroll_snap {
                        Some(PageScrollSnap::Mandatory) => "y mandatory",
                        Some(PageScrollSnap::Proximity) => "y proximity",
                        None => "none"
                    },
                    FILL
                ),
                { props.children }
            }
        }
    )
}