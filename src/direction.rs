#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(::strum_macros::Display)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::EnumCount)]
#[derive(::strum_macros::IntoStaticStr)]
#[derive(::strum_macros::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Direction {
    #[default]
    Bottom,
    BottomLeft,
    BottomRight,
    Top,
    TopLeft,
    TopRight,
    Left,
    Right
}