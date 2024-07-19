mod grid;
mod macros;

use std::{collections::HashSet, iter::once};

use colored::{ColoredString, Colorize};
use grid::*;

type Directions = Vec<Direction>;
type DirectedNeigbour = (Direction, Point);

type Path = Vec<Direction>;

fn last_three_items_are_the_same<T: PartialEq>(list: &Vec<T>) -> bool {
    match &list[..] {
        [.., a, b, c] if a == b && b == c => true,
        _ => false,
    }
}

struct Node {
    point: Option<Point>,
    heat_loss: Int,
    visited: bool,
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
                visited: false,
                shortest_distance_to_start: None,
                path_from_start: None,
            })
        };
    }

    fn assign_point(mut self, x: Int, y: Int) -> Node {
        self.point = Some(Point::new(x, y));
        self
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

    fn to_string(&self) -> String {
        let mut result = String::new();
        for y in 0..self.rows {
            for x in 0..self.columns {
                let node: &Node = &self.grid[y][x];
                let mut digit: ColoredString = node.heat_loss.to_string().into();
                let point: Point = node.point.clone().unwrap();
                if !self.unvisited.contains(&point) {
                    digit = digit.on_green();
                } else {
                    if node.path_from_start.is_some() && node.shortest_distance_to_start.is_some() {
                        digit = digit.on_blue();
                    } else {
                        digit = digit.on_red();
                    }
                }
                result.push_str(&digit.to_string());
            }
            result += "\n";
        }
        result
    }

    fn set_start_node(mut self, point: &Point) -> Map {
        let node = self.grid.point_get_mut(point).unwrap();
        node.shortest_distance_to_start = Some(0);
        node.path_from_start = Some(Vec::new());
        self
    }

    fn visit(mut self, point: &Point) -> Map {
        debug!(true, "visit({:?})", point);
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

        let directed_neighbours: Vec<DirectedNeigbour> = self.get_neighbours(point);
        for (direction_to_neighbour, point_of_neighbour) in directed_neighbours {
            let neighbour_node: &mut Node = self.grid.point_get_mut(&point_of_neighbour).unwrap();
            let path_to_neighbour: Path = path_from_start
                .iter()
                .cloned()
                .chain(once(direction_to_neighbour))
                .collect();
            if neighbour_node.path_from_start.is_some()
                || neighbour_node.shortest_distance_to_start.is_some()
            {
                debug!(
                    true,
                    "A shorter path from {:?} to {:?} already exists, skip.",
                    point,
                    point_of_neighbour
                );
            } else if last_three_items_are_the_same(&path_to_neighbour) {
                debug!(
                    true,
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

        self
    }

    // fn visit_old(
    //     mut self,
    //     point: &Point,
    //     shortest_distance_to_start: Int,
    //     path: Directions,
    // ) -> Map {
    //     if !self.unvisited.contains(point) {
    //         panic!(
    //             "Attempting to visit a Point that cannot be visited: '{:?}'",
    //             point
    //         );
    //     } else {
    //         self.unvisited.remove(point);
    //         let node = self.grid.point_get_mut(point).unwrap();
    //         node.shortest_distance_to_start = Some(shortest_distance_to_start);
    //         node.visited = true;
    //         let shortest_distance_to_neighbour = shortest_distance_to_start + node.heat_loss;
    //         let mut neighbours = self.get_neighbours(point);
    //         let mut neighbour_black_list: HashSet<Point> = HashSet::new();
    //         while !neighbours.is_empty() {
    //             let (direction, neighbour) = neighbours.first().unwrap();
    //             let path_to_neighbour: Vec<Direction> =
    //                 path.iter().chain(once(direction)).cloned().collect();

    //             let last_three_same = match path_to_neighbour.as_slice() {
    //                 [.., a, b, c] if a == b && b == c => true,
    //                 _ => false,
    //             };

    //             if last_three_same {
    //                 neighbour_black_list.insert(*neighbour);
    //             } else {
    //                 //TODO: This is not Dijkstra's algorithm! Need to visit the closest Nodes first, but a Neighbour blacklist is a good idea.
    //                 let neighbour_node = self.grid.point_get(neighbour).unwrap();
    //                 neighbour_node.shortest_distance_to_start = shortest_distance_to_start + self =
    //                     self.visit(
    //                         &neighbour,
    //                         shortest_distance_to_neighbour,
    //                         path_to_neighbour,
    //                     );
    //             }
    //             neighbours = self
    //                 .get_unvisited_neighbours(point)
    //                 .iter()
    //                 .filter(|(_, n)| !neighbour_black_list.contains(n))
    //                 .cloned()
    //                 .collect();
    //         }

    //         self
    //     }
    // }

    fn get_neighbours(&self, point: &Point) -> Vec<DirectedNeigbour> {
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
    let to_string = example_map.to_string();
    debug!(true, "example_map:\n{}", to_string);
    let shortest_distance_to_bottom_right = example_map
        .grid
        .wrapped_get(-1, -1)
        .unwrap()
        .shortest_distance_to_start;
    dbg!(shortest_distance_to_bottom_right);
    test!(102, "Part 1 - Example");
}
