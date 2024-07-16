pub type Int = i32;
pub type Grid<T> = Vec<Vec<T>>;

pub use Direction::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct Point {
    pub x: Int,
    pub y: Int,
}

impl Point {
    pub fn move_to(&self, direction: &Direction) -> Point {
        match direction {
            North => Point {
                x: self.x,
                y: self.y - 1,
            },
            East => Point {
                x: self.x + 1,
                y: self.y,
            },
            South => Point {
                x: self.x,
                y: self.y + 1,
            },
            West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    pub fn new(x: Int, y: Int) -> Point {
        Point { x, y }
    }
}
