pub mod abstraction;
pub mod model;

#[derive(Debug, Default)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
