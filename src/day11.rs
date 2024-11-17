use crate::*;

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
                None => write!(f, "Galaxy(None)"),
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
    galaxies: Vec<Data>,
    number_of_galaxies: Int,
}

impl Image {
    fn parse(input: Input) -> Image {
        let input: Vec<String> = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Image::from_strings(&input)
    }

    fn from_strings(input: &Vec<String>) -> Image {
        let data: Grid<Data> = input
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().map(|c| Data::from_char(c)).collect())
            .collect();
        let rows = data.len() as Int;
        let columns = data.first().unwrap().len() as Int;
        let mut image = Image {
            to_strings: input.clone(),
            data,
            rows,
            columns,
            galaxies: Vec::new(),
            number_of_galaxies: 0,
        };
        image.assign_positions_to_galaxies();
        image.galaxies = Image::find_galaxies(&image.data);
        image.number_of_galaxies = image.galaxies.len() as Int;
        image
    }

    fn find_galaxies(grid: &Grid<Data>) -> Vec<Data> {
        let mut result: Vec<Data> = Vec::new();
        for row in grid {
            for data in row {
                if let Data::Galaxy(_) = *data {
                    result.push(data.clone());
                }
            }
        }
        result
    }

    fn find_empty_rows(grid: &Grid<Data>) -> Vec<Int> {
        grid.iter()
            .enumerate()
            .filter(|(_, v)| v.iter().all(|d| *d == Data::EmptySpace))
            .map(|(i, _)| i as Int)
            .collect()
    }

    fn shift_values_by_index(sequence: &Vec<Int>, shift_by: Int) -> Vec<Int> {
        sequence
            .iter()
            .enumerate()
            .map(|(acc, i)| i + (acc as Int * shift_by))
            .collect()
    }

    fn get_empty_data_rows(&self, shift_by: Int) -> Vec<Int> {
        Image::shift_values_by_index(&Image::find_empty_rows(&self.data), shift_by)
    }

    fn get_empty_data_columns(&self, shift_by: Int) -> Vec<Int> {
        let data_transposed: Grid<Data> = transpose_grid(&self.data);
        Image::shift_values_by_index(&Image::find_empty_rows(&data_transposed), shift_by)
    }

    fn expand_universe(&self) -> Image {
        let empty_rows: Vec<Int> = self.get_empty_data_rows(1);
        let empty_columns: Vec<Int> = self.get_empty_data_columns(1);

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

    fn expand_universe_with_factor(&self, expansion_factor: Int) -> Image {
        let empty_rows: Vec<Int> = self.get_empty_data_rows(expansion_factor - 1);
        let empty_columns: Vec<Int> = self.get_empty_data_columns(expansion_factor - 1);
        let mut new_image = self.clone();

        for galaxy in &mut new_image.galaxies {
            match galaxy {
                Data::Galaxy(Some(position)) => {
                    for i in &empty_columns {
                        if position.x > *i {
                            // Need to shift this Galaxy
                            position.x += expansion_factor - 1;
                        }
                    }
                    for i in &empty_rows {
                        if position.y > *i {
                            // Need to shift this Galaxy
                            position.y += expansion_factor - 1;
                        }
                    }
                }
                _ => panic!("Invalid Galaxy: '{:?}'.", galaxy),
            }
        }

        new_image
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

    fn compute_distance_between_galaxies(a: &Data, b: &Data) -> Int {
        match (&a, &b) {
            (Data::Galaxy(Some(a_position)), Data::Galaxy(Some(b_position))) => {
                // Compute the Manhattan distance between a and b
                (a_position.x - b_position.x).abs() + (a_position.y - b_position.y).abs()
            }
            _ => panic!("These are not valid Galaxies: ({:?}, {:?}).", a, b),
        }
    }

    fn compute_sum_of_distances_between_all_galaxies(&self) -> Int {
        let mut sum = 0;
        for i in 0..self.number_of_galaxies {
            for j in 0..self.number_of_galaxies {
                if i == j {
                    // Skip compare with self
                    continue;
                }
                sum += Image::compute_distance_between_galaxies(
                    &self.galaxies[i as usize],
                    &self.galaxies[j as usize],
                );
            }
        }
        sum / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expansion() {
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
        let image = Image::test_expansion(&example_input, &example_input_expanded);
        test!(image.number_of_galaxies, 9);
        image.test_distance(1, 7, 15);
        image.test_distance(3, 6, 17);
        image.test_distance(8, 9, 5);
        image.test_sum_of_distances(374);
    }

    impl Image {
        fn test_image(expected: &Image, actual: &Image) -> bool {
            test!(
                expected.rows,
                actual.rows,
                "Rows compare failed: '{}' != '{}",
                expected.rows,
                actual.rows
            );
            test!(expected.data.len(), expected.rows as usize);
            test!(actual.data.len(), actual.rows as usize);
            test!(
                expected.columns,
                actual.columns,
                "Columns compare failed: '{}' != '{}",
                expected.columns,
                actual.columns
            );
            test!(
                expected.data.first().unwrap().len(),
                expected.columns as usize
            );
            test!(actual.data.first().unwrap().len(), actual.columns as usize);
            test!(
                expected.to_strings,
                actual.to_strings,
                "ToStrings compare failed."
            );
            test!(expected.number_of_galaxies, actual.number_of_galaxies);
            for y in 0..actual.rows {
                for x in 0..actual.columns {
                    let (x, y) = (x as usize, y as usize);
                    let expected_data = &expected.data[y][x];
                    let actual_data = &actual.data[y][x];
                    test!(expected_data, actual_data, "data({}, {})", x, y);
                }
            }
            test!(expected.data, actual.data, "Data compare failed.");
            true
        }

        fn test_expansion(input: &Vec<&str>, expected_expansion: &Vec<&str>) -> Image {
            let image = Image::from_strings(&input.iter().map(|s| s.to_string()).collect());
            dbg!(&image);
            let expected_expanded_mage =
                Image::from_strings(&expected_expansion.iter().map(|s| s.to_string()).collect());
            let actual_expanded_image = image.expand_universe();
            dbg!(&actual_expanded_image);
            Image::test_image(&expected_expanded_mage, &actual_expanded_image);
            actual_expanded_image
        }

        /// Test the distance between Galaxy a and Galaxy b
        fn test_distance(&self, galaxy_a: usize, galaxy_b: usize, expected_distance: Int) {
            let a = galaxy_a - 1;
            let b = galaxy_b - 1;
            test!(a < self.number_of_galaxies as usize);
            test!(b < self.number_of_galaxies as usize);
            let distance =
                Image::compute_distance_between_galaxies(&self.galaxies[a], &self.galaxies[b]);
            println!("Distance ({}, {}): {}", a + 1, b + 1, distance);
            test!(
                expected_distance,
                distance,
                "Test case failed (Galaxy {} -> {}): this distance should always equal '{}'.",
                a + 1,
                b + 1,
                expected_distance
            );
        }

        fn test_sum_of_distances(&self, expected_sum_of_distances: Int) {
            let sum_of_distances = self.compute_sum_of_distances_between_all_galaxies();
            test!(
                expected_sum_of_distances,
                sum_of_distances,
                "Test case failed: this value should always equal '{}'.",
                expected_sum_of_distances
            );
        }
    }
}

pub struct DayEleven {}

impl Problem for DayEleven {
    fn year(&self) -> Year {
        2023
    }
    fn day(&self) -> Day {
        11
    }
    fn expect_part_one(&self) -> Answer {
        9918828
    }
    fn expect_part_two(&self) -> Answer {
        692506533832
    }

    define_examples! {
        (
            "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            ",
            Expect::PartsOneAndTwo(374, 8410),
        )
    }

    fn solve_part_one(&self, input: Input, _is_example: bool) -> Answer {
        let image = Image::parse(input);
        let expanded_image = image.expand_universe();
        let sum_of_distances = expanded_image.compute_sum_of_distances_between_all_galaxies();
        sum_of_distances
    }

    fn solve_part_two(&self, input: Input, is_example: bool) -> Answer {
        let image = Image::parse(input);
        let scale = if is_example { 100 } else { 1_000_000 };
        let scaled_image = image.expand_universe_with_factor(scale);
        let sum_of_distances = scaled_image.compute_sum_of_distances_between_all_galaxies();
        sum_of_distances
    }
}
