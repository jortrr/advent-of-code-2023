use aoc::Grid;
use aoc::Int;
use std::fmt::Debug;
use std::fmt::Display;

/// Generic 2D grid with helper functions to move around
#[derive(Clone)]
pub struct Map<T> {
    pub grid: Grid<T>,
}

impl<T> Map<T> {
    pub fn get_rows(&self) -> usize {
        self.grid.len()
    }

    pub fn get_columns(&self) -> usize {
        self.grid.first().unwrap().len()
    }

    pub fn point_within_grid(&self, point: &Point) -> bool {
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

    /// Return a list of Points for which the Predicate on T hold
    pub fn find(&self, predicate: impl Fn(&T) -> bool) -> Vec<Point> {
        let mut result = Vec::new();
        for y in 0..self.get_rows() {
            for x in 0..self.get_columns() {
                let point = Point::new(x as Int, y as Int);
                let value = self.point_get(&point).unwrap();
                if predicate(value) {
                    result.push(point);
                }
            }
        }
        result
    }

    /// Returns a list of Points within the Map that are adjacent to point
    pub fn get_adjacent(&self, point: &Point) -> Vec<Point> {
        if !self.point_within_grid(point) {
            Vec::new()
        } else {
            vec![North, East, South, West]
                .iter()
                .map(|d| point.move_to(d))
                .filter(|p| self.point_within_grid(p))
                .collect()
        }
    }
}

// Deref and DerefMut allow us to extend a Grid<T> in Grid2D<T>
impl<T> Deref for Map<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<T> DerefMut for Map<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

// Implement FromIterator for Grid2D
impl<T> FromIterator<Vec<T>> for Map<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let grid: Vec<Vec<T>> = iter.into_iter().collect();
        Map { grid }
    }
}

use std::ops::{Deref, DerefMut};

use aoc::define_convertable_enum;
pub use Direction::*;

define_convertable_enum! {
    Direction {
        North => '^',
        East => '>',
        South => 'v',
        West => '<',
    }
}

impl Direction {
    pub fn move_left(&self) -> Direction {
        match &self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn move_right(&self) -> Direction {
        match &self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn move_back(&self) -> Direction {
        match &self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Clone, Eq, Hash, Copy, PartialOrd)]
pub struct Point {
    pub x: Int,
    pub y: Int,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Point {
    pub fn move_to(&self, direction: &Direction) -> Point {
        self.move_distance(direction, 1)
    }

    pub fn move_distance(&self, direction: &Direction, distance: Int) -> Point {
        match direction {
            North => Point {
                x: self.x,
                y: self.y - distance,
            },
            East => Point {
                x: self.x + distance,
                y: self.y,
            },
            South => Point {
                x: self.x,
                y: self.y + distance,
            },
            West => Point {
                x: self.x - distance,
                y: self.y,
            },
        }
    }

    pub fn moved_from(&self, direction: &Direction) -> Point {
        match direction {
            North => Point {
                x: self.x,
                y: self.y + 1,
            },
            East => Point {
                x: self.x - 1,
                y: self.y,
            },
            South => Point {
                x: self.x,
                y: self.y - 1,
            },
            West => Point {
                x: self.x + 1,
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

    pub fn translate(mut self, x: Int, y: Int) -> Point {
        self.x += x;
        self.y += y;
        self
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}
