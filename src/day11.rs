type Int = i32;

#[derive(Debug, PartialEq, Clone)]
struct Position {
    x: Int,
    y: Int,
}

#[derive(PartialEq, Clone)]
enum Data {
    EmptySpace,
    Galaxy(Option<Position>),
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySpace => write!(f, "EmptySpace"),
            Self::Galaxy(arg0) => match arg0 {
                Some(p) => write!(f, "Galaxy({}, {})", p.x, p.y),
                None => write!(f, "Galaxy"),
            },
        }
    }
}

impl Data {
    fn from_char(c: char) -> Data {
        use Data::*;
        match c {
            '.' => EmptySpace,
            '#' => Galaxy(None),
            _ => panic!("Not valid Data: '{}'.", c),
        }
    }

    fn to_char(&self) -> char {
        use Data::*;
        match *self {
            EmptySpace => '.',
            Galaxy(_) => '#',
        }
    }
}

type Grid<T> = Vec<Vec<T>>;

fn transpose_grid<T: Clone>(grid: &Grid<T>) -> Grid<T> {
    (0..grid.first().unwrap().len() - 1)
        .map(|i| {
            grid.iter()
                .map(move |r| r[i as usize].clone())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Image {
    to_strings: Vec<String>,
    data: Grid<Data>,
    rows: Int,
    columns: Int,
}

impl Image {
    fn from_strings(input: &Vec<String>) -> Image {
        let data: Grid<Data> = input
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().map(|c| Data::from_char(c)).collect())
            .collect();
        let rows = input.len() as Int;
        let columns = input.first().unwrap().len() as Int;
        let mut image = Image {
            to_strings: input.clone(),
            data,
            rows,
            columns,
        };
        image.assign_positions_to_galaxies();
        image
    }

    fn find_empty_rows(grid: &Grid<Data>) -> Vec<Int> {
        grid.iter()
            .enumerate()
            .filter(|(_, v)| v.iter().all(|d| *d == Data::EmptySpace))
            .map(|(i, _)| i as Int)
            .collect()
    }

    fn shift_values_by_index(sequence: &Vec<Int>) -> Vec<Int> {
        sequence
            .iter()
            .enumerate()
            .map(|(acc, i)| i + acc as Int)
            .collect()
    }

    fn expand_universe(&self) -> Image {
        let data_transposed: Grid<Data> = transpose_grid(&self.data);
        let empty_rows: Vec<Int> =
            Image::shift_values_by_index(&Image::find_empty_rows(&self.data));
        let empty_columns: Vec<Int> =
            Image::shift_values_by_index(&Image::find_empty_rows(&data_transposed));

        let mut input = self.to_strings.clone();
        for i in empty_rows {
            input.insert(
                i as usize,
                vec!['.'; self.columns as usize].into_iter().collect(),
            );
        }
        for i in empty_columns {
            for row in 0..input.len() {
                input[row].insert(i as usize, '.');
            }
        }

        Image::from_strings(&input)
    }

    fn assign_positions_to_galaxies(&mut self) {
        for x in 0..self.columns {
            for y in 0..self.rows {
                if let Data::Galaxy(position) = &mut self.data[y as usize][x as usize] {
                    *position = Some(Position { x, y })
                }
            }
        }
    }

    fn test_image(expected: &Image, actual: &Image) -> bool {
        dbg!(expected.rows);
        assert_eq!(
            expected.rows, actual.rows,
            "Rows compare failed: '{}' != '{}",
            expected.rows, actual.rows
        );
        assert_eq!(expected.data.len(), expected.rows as usize);
        assert_eq!(actual.data.len(), actual.rows as usize);
        dbg!(expected.columns);
        assert_eq!(
            expected.columns, actual.columns,
            "Columns compare failed: '{}' != '{}",
            expected.columns, actual.columns
        );
        assert_eq!(
            expected.data.first().unwrap().len(),
            expected.columns as usize
        );
        assert_eq!(actual.data.first().unwrap().len(), actual.columns as usize);
        dbg!(&expected.to_strings);
        assert_eq!(
            expected.to_strings, actual.to_strings,
            "ToStrings compare failed."
        );
        for y in 0..actual.rows {
            for x in 0..actual.columns {
                let (x, y) = (x as usize, y as usize);
                //dbg!((x, y));
                let expected_data = &expected.data[y][x];
                let actual_data = &actual.data[y][x];
                assert_eq!(
                    expected_data, actual_data,
                    "At data({}, {}): expected '{:?}' != actual '{:?}'.",
                    x, y, expected_data, actual_data
                );
            }
        }
        assert_eq!(expected.data, actual.data, "Data compare failed.");
        let data_comparison = "expected.data == actual.data";
        dbg!(data_comparison);
        true
    }

    fn test_expansion(input: &Vec<&str>, expected_expansion: &Vec<&str>) {
        let image = Image::from_strings(&input.iter().map(|s| s.to_string()).collect());
        dbg!(&image);
        let expected_expanded_mage =
            Image::from_strings(&expected_expansion.iter().map(|s| s.to_string()).collect());
        let actual_expanded_image = image.expand_universe();
        dbg!(&actual_expanded_image);
        Image::test_image(&expected_expanded_mage, &actual_expanded_image);
    }
}

fn main() {
    println!("Hello, World! from src/day11.rs!");
    // Example - Part 1
    let example_input: Vec<&str> = vec![
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];
    let example_input_expanded: Vec<&str> = vec![
        "....#........",
        ".........#...",
        "#............",
        ".............",
        ".............",
        "........#....",
        ".#...........",
        "............#",
        ".............",
        ".............",
        ".........#...",
        "#....#.......",
    ];
    Image::test_expansion(&example_input, &example_input_expanded);
}
