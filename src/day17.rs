mod grid;
mod macros;

use std::{collections::HashSet, iter::once};

use colored::{ColoredString, Colorize};
use grid::*;

type DirectedPoint = (Direction, Point);

type Path = Vec<DirectedPoint>;

static DEBUG: bool = true;

fn last_three_items_are_the_same<T: PartialEq>(list: &Vec<T>) -> bool {
    match &list[..] {
        [.., a, b, c] if a == b && b == c => true,
        _ => false,
    }
}

#[derive(Clone)]
struct Node {
    point: Option<Point>,
    heat_loss: Int,
    shortest_distance_to_start: Option<Int>,
    path_from_start: Option<Path>,
}

impl Node {
    fn from_digit(digit: Int) -> Option<Node> {
        return if digit < 0 || digit > 9 {
            None
        } else {
            Some(Node {
                point: None,
                heat_loss: digit,
                shortest_distance_to_start: None,
                path_from_start: None,
            })
        };
    }

    fn assign_point(mut self, x: Int, y: Int) -> Node {
        self.point = Some(Point::new(x, y));
        self
    }

    fn distance(&self) -> Option<Int> {
        match self.shortest_distance_to_start {
            Some(distance) => Some(distance + self.heat_loss),
            _ => None,
        }
    }
}

struct Map {
    rows: usize,
    columns: usize,
    grid: Grid<Node>,
    unvisited: HashSet<Point>,
}

impl Map {
    fn from_strings(input: &Vec<String>) -> Map {
        let grid: Grid<Node> = input
            .iter()
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        Node::from_digit(c.to_string().parse().unwrap())
                            .unwrap()
                            .assign_point(x as Int, y as Int)
                    })
                    .collect()
            })
            .collect();
        let unvisited: HashSet<Point> = grid.iter().flatten().map(|n| n.point.unwrap()).collect();
        let rows = grid.len();
        let columns = grid.first().unwrap().len();
        Map {
            rows,
            columns,
            grid,
            unvisited,
        }
    }

    fn to_string(&self, current_point_option: Option<Point>) -> String {
        let mut result = String::new();
        let mut path_to_current_node: Vec<Point> = Vec::new();
        if let Some(current_point) = current_point_option {
            path_to_current_node = self
                .grid
                .point_get(&current_point)
                .unwrap()
                .path_from_start
                .clone()
                .unwrap()
                .iter()
                .map(|(d, p)| *p)
                .collect();
        }
        for y in 0..self.rows {
            for x in 0..self.columns {
                let node: &Node = &self.grid[y][x];
                let mut digit: ColoredString = node.heat_loss.to_string().into();
                let point: Point = node.point.clone().unwrap();
                if !self.unvisited.contains(&point) {
                    digit = digit.on_green();
                } else {
                    if node.path_from_start.is_some() && node.shortest_distance_to_start.is_some() {
                        digit = digit.on_yellow();
                    } else {
                        digit = digit.on_red();
                    }
                }
                if let Some(current_point) = current_point_option {
                    if current_point == point {
                        digit = digit.on_blue();
                    }
                }
                if path_to_current_node.contains(&point) {
                    digit = digit.on_bright_magenta();
                }
                result.push_str(&digit.to_string());
            }
            result += "\n";
        }
        result
    }

    fn print(&self, current_point: &Point) {
        debug!(true, "Map:\n{}\n", self.to_string(Some(*current_point)));
    }

    fn set_start_node(mut self, point: &Point) -> Map {
        let node = self.grid.point_get_mut(point).unwrap();
        node.shortest_distance_to_start = Some(0);
        node.path_from_start = Some(Vec::new());
        self
    }

    fn get_closest_unvisited_node_point(&self) -> Option<Point> {
        let mut closest_node_option: Option<&Node> = None;

        for point in &self.unvisited {
            let node = self.grid.point_get(&point).unwrap();
            if closest_node_option.is_none() && node.distance().is_some() {
                closest_node_option = Some(node);
            } else if let Some(closest_node) = closest_node_option {
                match (node.distance(), closest_node.distance()) {
                    (Some(a), Some(b)) => {
                        if a < b {
                            closest_node_option = Some(node);
                        }
                    }
                    _ => (),
                }
            }
        }

        if let Some(clostest_node) = closest_node_option {
            debug!(
                DEBUG,
                "Found closest: {:?} -> {}.",
                clostest_node.point.unwrap(),
                clostest_node.distance().unwrap()
            );
            Some(clostest_node.point.unwrap())
        } else {
            None
        }
    }

    fn visit(mut self, point: &Point) -> Map {
        debug!(DEBUG, "visit({:?})", point);
        if !self.unvisited.contains(point) {
            panic!(
                "Attempting to visit a Point that is not in self.unvisited: '{:?}'.",
                point
            );
        }
        let node = self.grid.point_get(point).unwrap();
        if !node.shortest_distance_to_start.is_some() {
            panic!(
                "Attempting to visit a Node has no shortest_distance_to_start: '{:?}'.",
                point
            );
        }
        if !node.path_from_start.is_some() {
            panic!(
                "Attempting to visit a Node has no path_from_start: '{:?}'.",
                point
            );
        }
        // Now we know that our current point is unvisited, has a shortest_distance_to_start, and a path_from_start
        self.unvisited.remove(point);
        let shortest_distance_to_start: Int = node.shortest_distance_to_start.unwrap();
        let path_from_start: Path = node.path_from_start.clone().unwrap();
        let heat_loss: Int = node.heat_loss;

        let directed_neighbours: Vec<DirectedPoint> = self.get_neighbours(point);
        for (direction_to_neighbour, point_of_neighbour) in directed_neighbours {
            let neighbour_node: &mut Node = self.grid.point_get_mut(&point_of_neighbour).unwrap();
            let path_to_neighbour: Path = path_from_start
                .iter()
                .cloned()
                .chain(once((direction_to_neighbour, *point)))
                .collect();
            if neighbour_node.path_from_start.is_some()
                || neighbour_node.shortest_distance_to_start.is_some()
            {
                debug!(
                    false,
                    "A shorter path from {:?} to {:?} already exists, skip.",
                    point,
                    point_of_neighbour
                );
            } else if last_three_items_are_the_same(
                &path_to_neighbour.iter().map(|(d, _)| d).collect(),
            ) {
                debug!(
                    false,
                    "The last three directions in the Path from {:?} to {:?} are the same, skip.",
                    point,
                    point_of_neighbour
                );
            } else {
                // This neighbour node can be reached by the current node, and this is the shortest path
                neighbour_node.shortest_distance_to_start =
                    Some(shortest_distance_to_start + heat_loss);
                neighbour_node.path_from_start = Some(path_to_neighbour.clone());
            }
        }

        let next_node_point_option = self.get_closest_unvisited_node_point();

        if DEBUG {
            self.print(point);
        }

        if let Some(next_node_point) = next_node_point_option {
            self.visit(&next_node_point)
        } else {
            self
        }
    }

    fn get_neighbours(&self, point: &Point) -> Vec<DirectedPoint> {
        vec![North, East, South, West]
            .iter()
            .map(|d| (*d, point.move_to(d)))
            .filter(|(_, p)| self.grid.point_within(p))
            .collect()
    }
}

fn main() {
    println!("Hello, World! from src/day17.rs!\n");
    // Part 1 - Example
    let example_input = vec_of_strings![
        "2413432311323",
        "3215453535623",
        "3255245654254",
        "3446585845452",
        "4546657867536",
        "1438598798454",
        "4457876987766",
        "3637877979653",
        "4654967986887",
        "4564679986453",
        "1224686865563",
        "2546548887735",
        "4322674655533",
    ];
    let start_point: Point = Point::new(0, 0);
    let example_map = Map::from_strings(&example_input)
        .set_start_node(&start_point)
        .visit(&start_point);

    example_map.print(&example_map.grid.wrap(-1, -1));
    let shortest_distance_to_bottom_right = example_map
        .grid
        .wrapped_get(-1, -1)
        .unwrap()
        .shortest_distance_to_start
        .unwrap();
    test!(102, shortest_distance_to_bottom_right);
}
