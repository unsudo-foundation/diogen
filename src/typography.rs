use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Justify,
    Right
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Decor {
    Underline,
    Overline,
    LineThrough,
    Blink
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum DecorStyle {
    #[default]
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Default)]
pub enum Family {
    #[default]
    BrCobane
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Weight {
    #[default]
    Normal,
    Bold,
    Bolder,
    Lighter
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum WhiteSpace {
    #[default]
    Normal,
    NoWrap,
    Pre,
    PreWrap,
    PreLine
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum OverflowWrap {
    #[default]
    Normal,
    BreakWord,
    Anywhere
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(::strum_macros::Display)]
pub enum Gradient {
    #[default]
    #[strum(serialize = "linear-gradient")]
    Linear,
    #[strum(serialize = "radial-gradient")]
    Radial,
    #[strum(serialize = "conic-gradient")]
    Conic,
    #[strum(serialize = "repeating-gradient")]
    Repeating
}

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct TypographyProps<T> 
where
    T: 'static,
    T: Clone,
    T: PartialEq,
    T: Default,
    T: ::std::fmt::Display {
    pub size: Option<unit::Unit<unit::Relative>>,
    pub family: Option<T>,
    pub weight: Option<Weight>,
    pub gradient: Option<Gradient>,
    pub gradient_direction: Option<direction::Direction>,
    pub colors: Option<Vec<color::Color>>,
    pub letter_spacing: Option<unit::Unit<unit::Relative>>,
    pub word_spacing: Option<unit::Unit<unit::Relative>>,
    pub decor: Option<Vec<Decor>>,
    pub decor_color: Option<color::Color>,
    pub decor_style: Option<DecorStyle>,
    pub decor_thickness: Option<unit::Unit<unit::Relative>>,
    pub white_space: Option<WhiteSpace>,
    pub overflow_wrap: Option<OverflowWrap>,
    pub alignment: Option<Alignment>
}

/// # Example
/// ```rs
/// #[derive(Clone)]
/// #[derive(PartialEq)]
/// #[derive(Default)]
/// #[derive(::strum_macros::Display)]
/// #[strum(serialize_all = "kebab-case")]
/// enum Family {
///     SansSerif
/// }
/// 
/// function Main() -> Element {
///     rsx!(
///         ::diogen::typography::Typography::<Family> {
///             size: Some(("em", 1.0).into()),
///             gradient_direction: "bottom-right".into(),
///             colors: vec!(
///                 0x202020.into(),
///                 0xffffff.into()
///             ),
///             family: "sans-serif".into()
///         }
///     )
/// }
/// ```
#[component]
pub fn Typography<T>(props: TypographyProps<T>) -> Element 
where
    T: 'static,
    T: Clone,
    T: PartialEq,
    T: Default,
    T: ::std::fmt::Display {
    let size = props.size.unwrap_or_else(|| {
        (unit::Relative::Em, 1.0).into()
    });
    let family = props.family.unwrap_or_default();
    let weight  = props.weight.unwrap_or_default();
    let gradient = props.gradient.unwrap_or_default();
    let gradient_direction = props.gradient_direction.unwrap_or_default();
    let colors = props.colors.unwrap_or_else(|| {
        vec!(0xffffff.into())
    });
    let letter_spacing = props.letter_spacing.unwrap_or_else(|| {
        (unit::Relative::Em, 1.0).into()
    });
    let word_spacing = props.word_spacing.unwrap_or_else(|| {
        (unit::Relative::Em, 1.0).into()
    });
    let decor = props.decor
        .unwrap_or_default()
        .iter()
        .map(|decor| {
            format!("{}", decor)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let decor_color = props.decor_color.unwrap_or_default();
    let decor_style = props.decor_style.unwrap_or_default();
    let decor = format!("{} {} {}", decor, decor_style, decor_color);
    let white_space = props.white_space.unwrap_or_default();
    let overflow_wrap = props.overflow_wrap.unwrap_or_default();
    let alignment = props.alignment.unwrap_or_default();
    let mut background = String::new();
    background.push_str(&format!("{}", gradient));
    background.push('(');
    background.push_str(match gradient_direction {
        direction::Direction::Bottom => "to bottom",
        direction::Direction::BottomLeft => "to left bottom",
        direction::Direction::BottomRight => "to bottom right",
        direction::Direction::Left => "to left",
        direction::Direction::Right => "to right",
        direction::Direction::Top => "to top",
        direction::Direction::TopLeft => "to top left",
        direction::Direction::TopRight => "to top right"
    });
    background.push(',');
    background.push_str(
        &colors
            .iter()
            .map(|color| {
                format!("{}", color)
            })
            .collect::<Vec<_>>()
            .join(", ")
    );
    background.push(')');

    rsx!(
        div {
            style: format!(
                r#"
                    font-size: {};
                    font-family: {};
                    font-weight: {};
                    background: {};
                    letter-spacing: {};
                    word-spacing: {};
                    text-decoration: {};
                    white-space: {};
                    overflow-wrap: {};
                    text-align: {};
                "#,
                size,
                family,
                weight,
                background,
                letter_spacing,
                word_spacing,
                decor,
                white_space,
                overflow_wrap,
                alignment
            )
        }
    )
}