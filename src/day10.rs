use std::cmp::min;

type Int = i32;

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
type TilePosition = (Int, Int);
type TileAndPosition = (Option<Tile>, TilePosition);
type Neighbour = (TileAndPosition, Direction);
type Distances = Vec<Vec<Int>>;

struct Maze {
    to_strings: Vec<String>,
    maze: Vec<Tiles>,
    distances: Distances,
    rows: usize,
    columns: usize,
}

impl Maze {
    fn from_strings(tiles: &Vec<String>) -> Maze {
        let maze: Vec<Tiles> = tiles
            .iter()
            .map(|s| s.chars().map(|c| Tile::from_char(c)).collect())
            .collect();
        let rows = maze.len();
        let columns = maze.first().unwrap().len();
        let distances: Distances = vec![vec![-1; columns]; rows];

        Maze {
            to_strings: tiles.clone(),
            maze,
            distances,
            rows,
            columns,
        }
    }

    fn get_animal_starting_position(&self) -> Option<TilePosition> {
        for i in 0..(self.rows - 1) {
            for j in 0..(self.columns - 1) {
                let tile: &Tile = &self.maze[j][i];
                if *tile == Tile::AnimalStartingPosition {
                    return Some((i as Int, j as Int));
                }
            }
        }
        None
    }

    fn move_north(tile: TilePosition) -> TilePosition {
        (tile.0, tile.1 - 1)
    }

    fn move_east(tile: TilePosition) -> TilePosition {
        (tile.0 + 1, tile.1)
    }

    fn move_south(tile: TilePosition) -> TilePosition {
        (tile.0, tile.1 + 1)
    }

    fn move_west(tile: TilePosition) -> TilePosition {
        (tile.0 - 1, tile.1)
    }

    fn get_neighbour(&self, tile: TilePosition, direction: Direction) -> Neighbour {
        use Direction::*;

        let new_position = match direction {
            North => Maze::move_north(tile),
            East => Maze::move_east(tile),
            South => Maze::move_south(tile),
            West => Maze::move_west(tile),
        };

        (self.get_tile_and_position(new_position), direction)
    }

    fn get_tile(&self, position: TilePosition) -> Option<Tile> {
        if position.0 < 0
            || position.1 < 0
            || position.0 >= self.columns as Int
            || position.1 >= self.rows as Int
        {
            return None;
        }
        Some(self.maze[position.1 as usize][position.0 as usize].clone())
    }

    fn get_tile_and_position(&self, position: TilePosition) -> TileAndPosition {
        (self.get_tile(position), position)
    }

    fn update_distance(&mut self, distance: Int, position: TilePosition) {
        let x = position.0 as usize;
        let y = position.1 as usize;
        let current_distance = self.distances[y][x];
        let mut new_distance = distance;
        if current_distance >= 0 {
            new_distance = min(current_distance, distance);
        }
        self.distances[y][x] = new_distance;
    }

    fn find_longest_distance_from_animal_starting_position(&mut self) -> Int {
        use Direction::*;
        let start_position: TilePosition = self.get_animal_starting_position().unwrap();
        let mut current: TileAndPosition = self.get_tile_and_position(start_position);
        let mut visited: Vec<TilePosition> = Vec::new();
        let mut distance = 0;

        loop {
            let tile: Tile = current.0.clone().unwrap();
            let position: TilePosition = current.1;
            self.update_distance(distance, position);
            let mut travelled = false;
            println!(
                "[{}, {}]: {:?} (distance: {})",
                position.0, position.1, tile, distance
            );

            let north = self.get_neighbour(position, North);
            let east = self.get_neighbour(position, East);
            let south = self.get_neighbour(position, South);
            let west = self.get_neighbour(position, West);
            let neighbours = vec![north, east, south, west];

            for ((other_tile_option, other_position), direction) in neighbours {
                if let Some(other_tile) = other_tile_option {
                    if other_tile == Tile::AnimalStartingPosition && distance == 1 {
                        continue;
                    }
                    if tile.connected(&other_tile, direction) && !visited.contains(&other_position)
                    {
                        current = self.get_tile_and_position(other_position);
                        visited.push(current.1);
                        distance += 1;
                        travelled = true;
                        let tile = current.0.clone().unwrap();
                        if tile == Tile::AnimalStartingPosition {
                            println!(
                                "[{}, {}]: {:?} (distance: {})",
                                position.0, position.1, tile, distance
                            );
                            // Now, traverse the visited tiles in reverse, and update the Distances
                            let mut reverse_distance = 0;
                            for position in visited.iter().rev() {
                                self.update_distance(reverse_distance, *position);
                                reverse_distance += 1;
                            }
                            let max_distance = self
                                .distances
                                .iter()
                                .map(|v| v.iter().max().unwrap())
                                .max()
                                .unwrap();
                            return *max_distance;
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

        f.debug_struct("Maze")
            .field("to_strings", &self.to_strings)
            .field("maze", &self.maze)
            .field("distances", &distance_str)
            .field("rows", &self.rows)
            .field("columns", &self.columns)
            .finish()
    }
}

fn main() {
    println!("Hello, World! from src/day10.rs!");
    let input: Vec<String> = vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut maze = Maze::from_strings(&input);
    let distance = maze.find_longest_distance_from_animal_starting_position();
    dbg!(&maze);
    dbg!(distance);
    assert_eq!(8, distance, "This example distance should always be 8.")
}
