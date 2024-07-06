type Int = i32;

#[derive(Debug, PartialEq, Clone)]
struct Position {
    x: Int,
    y: Int,
}

#[derive(Debug, PartialEq, Clone)]
enum Data {
    EmptySpace,
    Galaxy(Option<Position>),
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
        Image {
            to_strings: input.clone(),
            data,
            rows,
            columns,
        }
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

        let mut data = self.data.clone();
        for i in empty_rows {
            data.insert(i as usize, vec![Data::EmptySpace; self.columns as usize]);
        }
        for i in empty_columns {
            for row in 0..data.len() {
                data[row].insert(i as usize, Data::EmptySpace);
            }
        }

        let rows = data.len() as i32;
        let columns = data.first().unwrap().len() as i32;
        let to_strings: Vec<String> = data
            .iter()
            .map(|r| r.iter().map(|d| d.to_char()).collect())
            .collect();

        Image {
            to_strings,
            data,
            rows,
            columns,
        }
    }

    fn test_image(expected: &Image, actual: &Image) -> bool {
        dbg!(expected.rows);
        assert_eq!(
            expected.rows, actual.rows,
            "Rows compare failed: '{}' != '{}",
            expected.rows, actual.rows
        );
        dbg!(expected.columns);
        assert_eq!(
            expected.columns, actual.columns,
            "Columns compare failed: '{}' != '{}",
            expected.columns, actual.columns
        );
        dbg!(&expected.to_strings);
        assert_eq!(
            expected.to_strings, actual.to_strings,
            "ToStrings compare failed."
        );
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
