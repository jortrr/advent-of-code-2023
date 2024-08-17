mod grid;
use grid::*;
mod problem;
use problem::*;

define_convertable_enum! {
    Terrain {
        Start => 'S',
        GardenPlot => '.',
        Rock => '#'
    }
}

struct Garden {
    map: Map<Terrain>,
}

impl Parse for Garden {
    fn parse(input: Input) -> Self {
        let mut map: Map<Terrain> = Map::<Terrain> { grid: Grid::new() };
        map.grid = input
            .lines()
            .map(|line| {
                line.to_string()
                    .chars()
                    .map(Terrain::from_char)
                    .collect::<Vec<_>>()
            })
            .collect();
        Garden { map }
    }
}

impl Debug for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = self
            .map
            .iter()
            .map(|v| v.iter().map(|t| Terrain::to_char(t)).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "\n{}", map)
    }
}

struct DayTwentyOne {}

impl Problem for DayTwentyOne {
    const YEAR: Year = 2023;
    const DAY: Day = 21;
    const PART_ONE_EXPECTED: Answer = 0;
    const PART_TWO_EXPECTED: Answer = 0;

    define_examples! {
        (
            "
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
            ",
            Expect::PartOne(16),
        )
    }

    fn solve_part_one(input: Input, is_example: bool) -> Answer {
        let garden = Garden::parse(input);
        dbg!(garden);
        0
    }

    fn solve_part_two(input: Input, is_example: bool) -> Answer {
        todo!()
    }
}

run!(DayTwentyOne);
