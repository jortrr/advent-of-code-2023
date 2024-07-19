pub type Int = i32;

#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn get_rows(&self) -> usize {
        self.grid.len()
    }

    pub fn get_columns(&self) -> usize {
        self.grid.first().unwrap().len()
    }

    pub fn point_within(&self, point: &Point) -> bool {
        self.within(point.x, point.y)
    }

    pub fn within(&self, x: Int, y: Int) -> bool {
        x >= 0 && x < self.get_columns() as Int && y >= 0 && y < self.get_rows() as Int
    }

    /// Wrap around bounds: keep x and y within the desired range x: [0, self.get_rows()) and y: [0, self.get_columns()).
    pub fn wrapped_point_get(&self, point: &Point) -> Option<&T> {
        let point = self.point_wrap_around_bounds(*point);
        self.get(point.x, point.y)
    }

    pub fn point_get(&self, point: &Point) -> Option<&T> {
        self.get(point.x, point.y)
    }

    pub fn point_get_mut(&mut self, point: &Point) -> Option<&mut T> {
        self.get_mut(point.x, point.y)
    }

    /// Wrap around bounds: keep x and y within the desired range x: [0, self.get_rows()) and y: [0, self.get_columns()).
    pub fn point_wrap_around_bounds(&self, point: Point) -> Point {
        point.point_wrap_around_bounds(self.get_columns() as Int, self.get_rows() as Int)
    }

    /// Wrap around bounds: keep x and y within the desired range x: [0, self.get_rows()) and y: [0, self.get_columns()).
    pub fn wrap(&self, x: Int, y: Int) -> Point {
        self.point_wrap_around_bounds(Point::new(x, y))
    }

    pub fn wrapped_get(&self, x: Int, y: Int) -> Option<&T> {
        self.point_get(&self.point_wrap_around_bounds(Point::new(x, y)))
    }

    pub fn get(&self, x: Int, y: Int) -> Option<&T> {
        return if self.within(x, y) {
            Some(&self.grid[y as usize][x as usize])
        } else {
            None
        };
    }

    pub fn get_mut(&mut self, x: Int, y: Int) -> Option<&mut T> {
        return if self.within(x, y) {
            Some(&mut self.grid[y as usize][x as usize])
        } else {
            None
        };
    }
}

// Deref and DerefMut allow us to extend a Grid<T> in Grid2D<T>
impl<T> Deref for Grid<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

// Implement FromIterator for Grid2D
impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let grid: Vec<Vec<T>> = iter.into_iter().collect();
        Grid { grid }
    }
}

use std::ops::{Deref, DerefMut};

pub use Direction::*;

#[derive(PartialEq, Debug, Clone, Eq, Hash, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash, Copy)]
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

    /// Wrap around bounds: keep x and y within the desired range [0, max_x) and [0, max_y).
    pub fn point_wrap_around_bounds(mut self, max_x: Int, max_y: Int) -> Point {
        self.x = if self.x >= 0 {
            self.x % max_x
        } else {
            (self.x % max_x + max_x) % max_x
        };

        self.y = if self.y >= 0 {
            self.y % max_y
        } else {
            (self.y % max_y + max_y) % max_y
        };
        self
    }
}
