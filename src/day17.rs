mod grid;
mod macros;
mod path_finding;

use std::fmt::{Debug, Display};
use std::{collections::HashSet, iter::once, thread::sleep, time::Duration};

use colored::{ColoredString, Colorize};
use grid::*;
use path_finding::{Graph, NodeRef};

type DirectedPoint = (Direction, Point);

type Path = Vec<DirectedPoint>;
type Steps = Int;

#[derive(Debug, PartialEq, Clone)]
struct State {
    point: Point,
    direction: Direction,
    steps: Steps,
}

impl State {
    fn new(point: Point, direction: Direction, steps: Steps) -> State {
        State {
            point,
            direction,
            steps,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(p: {}, d: {}, n: {})",
            self.point, self.direction, self.steps
        )
    }
}

static DEBUG: bool = true;

#[derive(Clone, Debug)]
struct Node {
    point: Point,
    heat_loss: Int,
}

impl Node {
    fn from_digit(digit: Int, x: Int, y: Int) -> Node {
        if digit < 0 || digit > 9 {
            panic!("Digit not in range [0,9]: '{}'.", digit);
        }
        Node {
            point: Point::new(x, y),
            heat_loss: digit,
        }
    }
}

#[derive(Debug)]
struct Map {
    rows: usize,
    columns: usize,
    grid: Grid<Node>,
    graph: Graph<State>,
}

impl Map {
    fn from_strings(input: &Vec<String>, starting_state: State) -> Map {
        let grid: Grid<Node> = input
            .iter()
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        Node::from_digit(c.to_string().parse().unwrap(), x as Int, y as Int)
                    })
                    .collect()
            })
            .collect();
        let rows = grid.len();
        let columns = grid.first().unwrap().len();
        Map {
            rows,
            columns,
            grid,
            graph: Graph::new(starting_state),
        }
    }

    fn to_string(&self, current_point_option: Option<Point>) -> String {
        let mut result = String::new();
        //let mut path_to_current_node = self.graph.get_path(state)
        for y in 0..self.rows {
            for x in 0..self.columns {
                let node: &Node = &self.grid[y][x];
                let digit: ColoredString = node.heat_loss.to_string().into();
                //let point: Point = node.point;
                /*if !self.graph.contains(&point) {
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
                }*/
                result.push_str(&digit.to_string());
            }
            result += "\n";
        }
        result
    }

    fn print(&self, current_point: &Point) {
        debug!(
            true,
            "Map(distance to {:?}: {}):\n{}\n",
            current_point,
            -1, //TODO
            self.to_string(Some(*current_point))
        );
    }

    /// Generate all possible edges from any point inside the grid to any other point, and add them to self.graph
    fn generate_edges(&mut self) {
        for y in 0..self.rows {
            for x in 0..self.columns {
                let point: Point = Point::new(x as Int, y as Int);
                let node: &Node = self.grid.point_get(&point).unwrap();
                for direction in [North, East, South, West] {
                    let other_point = point.moved_from(&direction);
                    if self.grid.point_within_grid(&other_point) {
                        for i in 0..2 {
                            // Move in the same direction, limited to 3 times the same direction
                            let other_state: State = State::new(other_point, direction, i);
                            let state: State = State::new(point, direction, i + 1);
                            self.graph
                                .add_edge(other_state.clone(), state, node.heat_loss);

                            // Bend left or right
                            let bend_left_state = State::new(point, direction.move_left(), 0);
                            let bend_right_state = State::new(point, direction.move_right(), 0);
                            self.graph.add_edge(
                                other_state.clone(),
                                bend_left_state.clone(),
                                node.heat_loss,
                            );
                            self.graph
                                .add_edge(other_state, bend_right_state, node.heat_loss);
                        }
                        // Bend left or right, because the other has to
                        let other_state: State = State::new(other_point, direction, 2);
                        let bend_left_state = State::new(point, direction.move_left(), 0);
                        let bend_right_state = State::new(point, direction.move_right(), 0);
                        self.graph.add_edge(
                            other_state.clone(),
                            bend_left_state.clone(),
                            node.heat_loss,
                        );
                        self.graph
                            .add_edge(other_state, bend_right_state, node.heat_loss);
                    }
                }
            }
        }
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
    let mut example_map = Map::from_strings(&example_input, State::new(Point::new(0, 0), South, 0));
    example_map.generate_edges();
    example_map.graph.run_pathfinding_algorithm();
    let destination = example_map.grid.wrap(-1, -1);
    let mut paths: Vec<NodeRef<State>> = example_map
        .graph
        .visited_nodes
        .iter()
        .filter(|n| n.borrow().state.point == destination)
        .cloned()
        .collect();
    dbg!(&paths);
    paths.sort_by(|a, b| {
        a.borrow()
            .distance_option
            .unwrap()
            .partial_cmp(&b.borrow().distance_option.unwrap())
            .unwrap()
    });
    let shortest_path_to_point = paths.first().unwrap();
    let shortest_distance_to_point = shortest_path_to_point.borrow().distance_option.unwrap();
    dbg!(shortest_path_to_point);
    dbg!(shortest_distance_to_point);

    //dbg!(example_map);
    test!(102, shortest_distance_to_point, "Part 1 - Example");
}
