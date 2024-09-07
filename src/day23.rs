mod grid;
mod problem;

use std::{cmp::max, collections::HashSet};

use grid::*;
use problem::*;

struct Node {
    point: Point,
    neighbours: Vec<Point>,
}

type Nodes = Vec<Node>;
type NodeMap = HashMap<Point, Node>;

fn get_nodes(grid: Grid<char>) -> Nodes {
    let mut nodes = Nodes::new();
    let max_y = grid.len();
    let max_x = grid.first().unwrap().len();
    for y in 0..max_y {
        for x in 0..max_x {
            let c: char = grid[y][x];
            if c == '#' {
                continue;
            }
            let p: Point = Point::new(x as Int, y as Int);
            let neighbours: Vec<Point> = match c {
                '.' => vec![North, East, South, West],
                '^' => vec![North],
                '>' => vec![East],
                '<' => vec![West],
                'v' => vec![South],
                _ => unreachable!(),
            }
            .iter()
            .map(|d| p.move_to(d))
            .filter(|p| p.in_grid(max_x as Int - 1, max_y as Int - 1))
            .filter(|p| grid[p.y as usize][p.x as usize] != '#')
            .collect();
            nodes.push(Node {
                point: p,
                neighbours,
            });
        }
    }
    nodes
}

fn get_first_path_in_row(grid: &Grid<char>, row: usize) -> Point {
    let find_index_of_a_path = |row: &Vec<char>| {
        row.iter()
            .enumerate()
            .filter(|(_, &c)| c == '.')
            .map(|(i, _)| i)
            .nth(0)
            .unwrap()
    };
    Point::new(
        find_index_of_a_path(grid.get(row).unwrap()) as Int,
        row as Int,
    )
}

/// Find longest path from current_node to target_node using DFS + backtracking
fn find_longest_path(
    node_map: &NodeMap,
    current_node: &Point,
    target_node: &Point,
    visited: &mut HashSet<Point>,
    current_path: Int,
    longest_path: &mut Int,
) {
    // If we reach the target node, update the max_length
    if current_node == target_node {
        *longest_path = max(current_path, *longest_path);
        return;
    }

    // Mark the current node as visited
    visited.insert(*current_node);
}

struct DayTwentyThree {}

impl Problem for DayTwentyThree {
    const YEAR: Year = 2023;
    const DAY: Day = 23;
    const PART_ONE_EXPECTED: Answer = 0;
    const PART_TWO_EXPECTED: Answer = 0;

    define_examples! {
        (
        "
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
        ",
        Expect::PartOne(94),
        )
    }

    fn solve_part_one(input: Input, is_example: bool) -> Answer {
        let grid: Grid<char> = grid_of_chars!(input);
        dbg!(&grid.len());
        let start: Point = get_first_path_in_row(&grid, 0);
        let end: Point = get_first_path_in_row(&grid, grid.len() - 1);
        debug!(
            is_example,
            "Find longest path: from '{}' to '{}'", start, end
        );
        let nodes: Nodes = get_nodes(grid);
        let node_map: HashMap<Point, &Node> = nodes.iter().map(|n| (n.point, n)).collect();
        // DFS
        let mut v: Vec<&Point> = Vec::new();
        let mut s: Stack<StackInfo> = Stack::from(vec![(start, start)]);
        while let Some(p) = s.pop() {
            if !v.contains(&p) {
                v.push(p);
                let c: &Node = node_map[&p];
                for n in &c.neighbours {
                    if !s.contains(&n) {
                        s.push(n);
                    }
                }

                if *p == end {
                    debug!(is_example, "{} -> {}: {}", start, end, v.len() - 1);
                }
            }
        }
        0
    }

    fn solve_part_two(input: Input, is_example: bool) -> Answer {
        todo!()
    }
}

run!(DayTwentyThree);
