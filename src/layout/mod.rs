use super::*;

::modwire::expose!(
    pub col
    pub grid_item
    pub grid
    pub page_item
    pub page
    pub row
    pub stack_item
    pub stack
);

static ABS_POS_RESET: &str = r#"
    top: 0%;
    left: 0%;
"#;

static FILL_VIEW: &str = r#"
    min-width: 100vw;
    max-width: 100vw;
    width: 100vw;
    min-height: 100vh;
    max-height: 100vh;
    height: 100vh;
"#;

static FILL: &str = r#"
    width: 100%;
    height: 100%;
    flex: 1;
"#;

#[derive(Props, Clone, PartialEq)]
pub struct CommonProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}