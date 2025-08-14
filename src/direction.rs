#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Default)]
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