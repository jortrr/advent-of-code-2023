use std::{
    cmp::{max, min},
    fmt::format,
};

static PRINT_DISTANCES: bool = false;
static RUN_PART_1: bool = false;

type Int = i32;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    NorthSouthPipe,
    EastWestPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    Ground,
    AnimalStartingPosition,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        use Tile::*;
        match c {
            '|' => NorthSouthPipe,
            '-' => EastWestPipe,
            'L' => NorthEastPipe,
            'J' => NorthWestPipe,
            '7' => SouthWestPipe,
            'F' => SouthEastPipe,
            '.' => Ground,
            'S' => AnimalStartingPosition,
            'O' | 'I' => Ground,
            _ => panic!("Invalid Tile: '{}'.", c),
        }
    }

    fn goes(&self, direction: Direction) -> bool {
        use Direction::*;
        use Tile::*;
        let tile = self.clone();
        return match direction {
            North => match tile {
                NorthSouthPipe | NorthEastPipe | NorthWestPipe | AnimalStartingPosition => true,
                _ => false,
            },
            East => match tile {
                EastWestPipe | NorthEastPipe | SouthEastPipe | AnimalStartingPosition => true,
                _ => false,
            },
            South => match tile {
                NorthSouthPipe | SouthEastPipe | SouthWestPipe | AnimalStartingPosition => true,
                _ => false,
            },
            West => match tile {
                EastWestPipe | NorthWestPipe | SouthWestPipe | AnimalStartingPosition => true,
                _ => false,
            },
        };
    }

    fn connected(&self, other: &Tile, direction_of_other: Direction) -> bool {
        use Direction::*;
        let from: Tile = self.clone();
        let to: Tile = other.clone();
        match direction_of_other {
            North => from.goes(North) && to.goes(South),
            East => from.goes(East) && to.goes(West),
            South => from.goes(South) && to.goes(North),
            West => from.goes(West) && to.goes(East),
        }
    }
}

type Tiles = Vec<Tile>;
type Position = (Int, Int);
type TileAndPosition = (Option<Tile>, Position);
type Neighbour = (TileAndPosition, Direction);
type Distances = Vec<Vec<Int>>;

struct Maze {
    to_strings: Vec<String>,
    maze: Vec<Tiles>,
    distances: Distances,
    visited: Vec<Position>,
    rows: usize,
    columns: usize,
}

impl Maze {
    fn from_strings(tiles: &Vec<String>) -> Maze {
        let maze: Vec<Tiles> = tiles
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().map(|c| Tile::from_char(c)).collect())
            .collect();
        let rows = maze.len();
        let columns = maze.first().unwrap().len();
        let distances: Distances = vec![vec![-1; columns]; rows];

        Maze {
            to_strings: tiles.clone(),
            maze,
            distances,
            visited: Vec::new(),
            rows,
            columns,
        }
    }

    fn get_animal_starting_position(&self) -> Option<Position> {
        for i in 0..(self.columns - 1) {
            for j in 0..(self.rows - 1) {
                let tile: &Tile = &self.maze[j][i];
                if *tile == Tile::AnimalStartingPosition {
                    return Some((i as Int, j as Int));
                }
            }
        }
        None
    }

    fn move_north(tile: Position) -> Position {
        (tile.0, tile.1 - 1)
    }

    fn move_east(tile: Position) -> Position {
        (tile.0 + 1, tile.1)
    }

    fn move_south(tile: Position) -> Position {
        (tile.0, tile.1 + 1)
    }

    fn move_west(tile: Position) -> Position {
        (tile.0 - 1, tile.1)
    }

    fn get_neighbour(&self, tile: Position, direction: Direction) -> Neighbour {
        use Direction::*;

        let new_position = match direction {
            North => Maze::move_north(tile),
            East => Maze::move_east(tile),
            South => Maze::move_south(tile),
            West => Maze::move_west(tile),
        };

        (self.get_tile_and_position(new_position), direction)
    }

    fn get_tile(&self, position: Position) -> Option<Tile> {
        if position.0 < 0
            || position.1 < 0
            || position.0 >= self.columns as Int
            || position.1 >= self.rows as Int
        {
            return None;
        }
        Some(self.maze[position.1 as usize][position.0 as usize].clone())
    }

    fn get_tile_and_position(&self, position: Position) -> TileAndPosition {
        (self.get_tile(position), position)
    }

    fn update_distance(&mut self, distance: Int, position: Position) -> Int {
        let x = position.0 as usize;
        let y = position.1 as usize;
        let current_distance = self.distances[y][x];
        let mut new_distance = distance;
        if current_distance >= 0 {
            new_distance = min(current_distance, distance);
        }
        self.distances[y][x] = new_distance;
        new_distance
    }

    fn get_area_enclosed_by_visited_tiles(&self) -> f64 {
        // We use the Shoelace formule to find this area
        // See: https://en.wikipedia.org/wiki/Shoelace_formula

        let circular_visited: Vec<Position> = self
            .visited
            .iter()
            .chain(vec![self.visited.first().unwrap()])
            .cloned()
            .collect();
        let area: f64 = circular_visited
            .windows(2)
            .map(|v| {
                if let [a, b] = v {
                    (a.0 * b.1 - b.0 * a.1) as f64
                } else {
                    panic!("This should never happen")
                }
            })
            .sum::<f64>()
            / 2.0;
        area
    }

    fn get_interior_points(&self) -> Int {
        // Use Pick's theorem to find the interior points from the area and boundary points
        // Let i be the number of integer points interior to the polygon
        // Let b be the number of integer points on its boundary
        // Then the area A of this polygon is: A = i + b/2 - 1
        // So then i = A - b/2 + 1
        let area = self.get_area_enclosed_by_visited_tiles();
        let boundary_points = self.visited.len();
        let interior_points = area - (boundary_points as f64 / 2.0) + 1.0;
        interior_points as Int
    }

    fn print_visit_distance_to_start(position: &Position, tile: &Tile, distance: i32) {
        if PRINT_DISTANCES {
            println!(
                "[{}, {}]: {:?} (distance: {})",
                position.0, position.1, tile, distance
            );
        }
    }

    fn find_longest_distance_from_animal_starting_position(&mut self) -> Int {
        use Direction::*;
        let mut current: TileAndPosition =
            self.get_tile_and_position(self.get_animal_starting_position().unwrap());
        let mut distance = 0;

        loop {
            let (tile, position) = (current.0.clone().unwrap(), current.1);
            let mut travelled = false;
            self.update_distance(distance, position);
            Maze::print_visit_distance_to_start(&position, &tile, distance);

            let neighbours: Vec<Neighbour> = vec![
                self.get_neighbour(position, North),
                self.get_neighbour(position, East),
                self.get_neighbour(position, South),
                self.get_neighbour(position, West),
            ];

            for ((other_tile_option, other_position), direction) in neighbours {
                if let Some(other_tile) = other_tile_option {
                    if other_tile == Tile::AnimalStartingPosition && distance == 1 {
                        // Do not return to start at first visit
                        continue;
                    }
                    if tile.connected(&other_tile, direction)
                        && !self.visited.contains(&other_position)
                    {
                        current = self.get_tile_and_position(other_position);
                        self.visited.push(current.1);
                        distance += 1;
                        travelled = true;
                        let tile = current.0.clone().unwrap();
                        if tile == Tile::AnimalStartingPosition {
                            Maze::print_visit_distance_to_start(&position, &tile, distance);
                            // Now, traverse the visited tiles in reverse, and update the Distances
                            let mut max_distance = 0;
                            for (distance, position) in
                                self.visited.clone().iter().rev().enumerate()
                            {
                                let new_distance = self.update_distance(distance as Int, *position);
                                max_distance = max(max_distance, new_distance);
                            }
                            return max_distance;
                        }
                        break;
                    }
                }
            }
            if !travelled {
                panic!("No more neighbours to go to, stuck!");
            }
        }
    }

    // Test case for Example Part 1
    fn test_distance(input: Vec<&str>, expected_distance: Int) {
        let mut maze = Maze::from_strings(&input.iter().map(|s| s.to_string()).collect());
        let distance = maze.find_longest_distance_from_animal_starting_position();
        dbg!(maze.to_strings);
        dbg!(distance);
        assert_eq!(
            expected_distance, distance,
            "Test case failed: this value should always equal '{}'.",
            expected_distance
        );
    }

    // Test case for Example Part 2
    fn test_interior_points(input: Vec<&str>, expected_interior_points: Int) {
        let mut maze = Maze::from_strings(&input.iter().map(|s| s.to_string()).collect());
        let _ = maze.find_longest_distance_from_animal_starting_position();
        let interior_points = maze.get_interior_points();
        dbg!(maze.to_strings);
        dbg!(interior_points);
        assert_eq!(
            expected_interior_points, interior_points,
            "Test case failed: this value should always equal '{}'.",
            expected_interior_points
        );
    }
}

impl std::fmt::Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let distance_str: Vec<String> = self
            .distances
            .iter()
            .map(|distances| {
                distances
                    .iter()
                    .map(|x| format!("[{}]", x).to_string())
                    .collect()
            })
            .collect();
        let visited_str: String = self
            .visited
            .iter()
            .map(|p| format!("[{}, {}]", p.0, p.1).to_string())
            .collect::<Vec<String>>()
            .join(" -> ");

        f.debug_struct("Maze")
            .field("to_strings", &self.to_strings)
            .field("maze", &self.maze)
            .field("distances", &distance_str)
            .field("visited", &visited_str)
            .field("rows", &self.rows)
            .field("columns", &self.columns)
            .finish()
    }
}

fn main() {
    println!("Hello, World! from src/day10.rs!");

    // Part 1 - Example
    #[rustfmt::skip]
    let example_input_4_d_a: Vec<&str> = vec![
        ".....",
        ".S-7.",
        ".|.|.",
        ".L-J.",
        ".....",
    ];
    Maze::test_distance(example_input_4_d_a, 4);
    #[rustfmt::skip]
    let example_input_4_d_b: Vec<&str> = vec![
        "-L|F7",
        "7S-7|",
        "L|7||",
        "-L-J|",
        "L|-JF",
    ];
    Maze::test_distance(example_input_4_d_b, 4);
    #[rustfmt::skip]
    let example_input_8_d_a: Vec<&str> = vec![
        "..F7.",
        ".FJ|.",
        "SJ.L7",
        "|F--J",
        "LJ...",
    ];
    Maze::test_distance(example_input_8_d_a, 8);
    #[rustfmt::skip]
    let example_input_8_d_b: Vec<&str> = vec![
        "7-F7-",
        ".FJ|7",
        "SJLL7",
        "|F--J",
        "LJ.LJ",
    ];
    Maze::test_distance(example_input_8_d_b, 8);

    // Part 2 - example
    let example_input_4_i_a: Vec<&str> = vec![
        "...........",
        ".S-------7.",
        ".|F-----7|.",
        ".||.....||.",
        ".||.....||.",
        ".|L-7.F-J|.",
        ".|..|.|..|.",
        ".L--J.L--J.",
        "...........",
    ];
    Maze::test_interior_points(example_input_4_i_a, 4);
    let example_input_4_i_b: Vec<&str> = vec![
        "..........",
        ".S------7.",
        ".|F----7|.",
        ".||OOOO||.",
        ".||OOOO||.",
        ".|L-7F-J|.",
        ".|II||II|.",
        ".L--JL--J.",
        "..........",
    ];
    Maze::test_interior_points(example_input_4_i_b, 4);
    let example_input_8_i: Vec<&str> = vec![
        "OF----7F7F7F7F-7OOOO",
        "O|F--7||||||||FJOOOO",
        "O||OFJ||||||||L7OOOO",
        "FJL7L7LJLJ||LJIL-7OO",
        "L--JOL7IIILJS7F-7L7O",
        "OOOOF-JIIF7FJ|L7L7L7",
        "OOOOL7IF7||L7|IL7L7|",
        "OOOOO|FJLJ|FJ|F7|OLJ",
        "OOOOFJL-7O||O||||OOO",
        "OOOOL---JOLJOLJLJOOO",
    ];
    Maze::test_interior_points(example_input_8_i, 8);

    // Part 1
    if RUN_PART_1 {
        let input = aoc_input::get(2023, 10);
        let mut maze = Maze::from_strings(&input);
        let distance = maze.find_longest_distance_from_animal_starting_position();
        dbg!(distance);
    }

    // Part 2
    //let interior_points = maze.get_interior_points();
    //dbg!(interior_points);
}
