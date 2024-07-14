type Int = i32;
type Grid<T> = Vec<Vec<T>>;

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

#[derive(PartialEq, Debug, Clone)]
struct Point {
    x: Int,
    y: Int,
}

impl Point {
    fn move_to(&self, direction: Direction) -> Point {
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

    fn new(x: Int, y: Int) -> Point {
        Point { x, y }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Terrain {
    RoundedRock(Option<Point>),
    CubeShapedRock(Option<Point>),
    EmptySpace(Option<Point>),
}

impl Terrain {
    fn from_char(c: char) -> Terrain {
        use Terrain::*;
        match c {
            'O' => RoundedRock(None),
            '#' => CubeShapedRock(None),
            '.' => EmptySpace(None),
            _ => panic!("Invalid Terrain char: '{}'.", c),
        }
    }
}

#[derive(Debug)]
struct Platform {
    rows: usize,
    columns: usize,
    grid: Grid<Terrain>,
}

impl Platform {
    fn tilt(&mut self, direction: Direction) {
        match direction {
            North => {
                for y in 0..self.rows {
                    for x in 0..self.columns {
                        let point = Point::new(x as Int, y as Int);
                        while self.can_move(&point, North) {
                            self.move_to(&point, North);
                        }
                    }
                }
            }
            _ => panic!("Platform::tilt({:?}) not implemented.", direction),
        }
    }

    fn move_to(&mut self, from_point: &Point, direction: Direction) {
        let from = self.get(from_point).unwrap().clone();
        let to = self.get(&from_point.move_to(direction)).unwrap().clone();
        match (&from, &to) {
            (Terrain::RoundedRock(_), Terrain::EmptySpace(Some(to_point))) => {
                self.set(&to_point, &from);
                self.set(from_point, &to);
            }
            _ => panic!("Not able to move from '{:?}' to '{:?}'.", from, to),
        }
    }

    fn can_move(&self, point: &Point, direction: Direction) -> bool {
        let other = self.get(&point.move_to(direction));
        match other {
            Some(Terrain::EmptySpace(_)) => true,
            _ => false,
        }
    }

    fn point_inside_grid(&self, point: &Point) -> bool {
        point.x < 0 || point.x >= self.columns as Int || point.y < 0 || point.y >= self.rows as Int
    }

    fn get(&self, point: &Point) -> Option<&Terrain> {
        let point_outside_grid = self.point_inside_grid(point);
        if point_outside_grid {
            None
        } else {
            Some(&self.grid[point.y as usize][point.x as usize])
        }
    }

    fn set(&mut self, point: &Point, terrain: &Terrain) {
        assert!(self.point_inside_grid(point));
        self.grid[point.y as usize][point.x as usize] = terrain.clone();
    }

    fn load_at_row(&self, row: Int) -> Option<Int> {
        if row < 0 || row >= self.rows as Int {
            None
        } else {
            Some(self.rows as Int - row)
        }
    }

    fn get_total_load(&self) -> Int {
        let mut total_load = 0;
        for y in 0..self.rows {
            let load_at_row = self.load_at_row(y as Int).unwrap();
            for x in 0..self.columns {
                let point = Point::new(x as Int, y as Int);
                let terrain = self.get(&point).unwrap();
                match terrain {
                    Terrain::RoundedRock(_) => total_load += load_at_row,
                    _ => (),
                }
            }
        }
        total_load
    }

    fn assign_points(&mut self) {
        //TODO: Continue here
        for y in 0..self.rows {
            for x in 0..self.columns {
                let point = Point::new(x as Int, y as Int);
                //self.grid[y][x]
            }
        }
    }

    fn from_string_slices(input: Vec<&str>) -> Platform {
        let input_strings = input.iter().map(|s| s.to_string()).collect();
        Platform::from_strings(input_strings)
    }

    fn from_strings(input: Vec<String>) -> Platform {
        let grid: Grid<Terrain> = input
            .iter()
            .map(|s| s.chars().map(|c| Terrain::from_char(c)).collect())
            .collect();
        let rows = grid.len();
        let columns = grid.first().unwrap().len();
        Platform {
            rows,
            columns,
            grid,
        }
    }
}

fn main() {
    println!("Hello, World! from src/day14.rs!");
    // Part 1 - Example
    let example_input = vec![
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ];
    let example_platform = Platform::from_string_slices(example_input);
    dbg!(example_platform);
}
