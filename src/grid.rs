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

    pub fn point_get(&self, point: &Point) -> Option<&T> {
        self.get(point.x, point.y)
    }

    pub fn get(&self, x: Int, y: Int) -> Option<&T> {
        return if self.within(x, y) {
            Some(&self.grid[y as usize][x as usize])
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

    pub fn new(x: Int, y: Int) -> Point {
        Point { x, y }
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
