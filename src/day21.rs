mod grid;
mod problem;

use grid::*;
use problem::*;

type GardenMap = grid::Map<Terrain>;

define_convertable_enum! {
    Terrain {
        Start => 'S',
        GardenPlot => '.',
        Rock => '#',
        PossibleLocation => 'O',
    }
}

#[derive(Clone, Debug)]
struct GardenMapRecord {
    map: GardenMap,
    at_step: Int,
}

#[derive(Debug)]
struct Garden {
    map: GardenMap,
    map_history: Vec<GardenMapRecord>,
}

impl Parse for Garden {
    fn parse(input: Input) -> Self {
        let map: GardenMap = GardenMap {
            grid: input
                .lines()
                .map(|line| {
                    line.to_string()
                        .chars()
                        .map(Terrain::from_char)
                        .collect::<Vec<_>>()
                })
                .collect(),
        };
        Garden {
            map: map.clone(),
            map_history: vec![GardenMapRecord { map, at_step: 0 }],
        }
    }
}

impl Debug for GardenMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = self
            .iter()
            .map(|v| v.iter().map(|t| Terrain::to_char(t)).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "\n{}", map)
    }
}

impl Garden {
    fn step_once(&mut self) {
        let last_record = self.map_history.last().unwrap();
        let mut next_record: GardenMapRecord = GardenMapRecord {
            map: self.map.clone(),
            at_step: last_record.at_step + 1,
        };
        next_record.map = next_record
            .map
            .iter()
            .map(|v| {
                v.iter()
                    .map(|t| {
                        if *t == Terrain::Start {
                            Terrain::GardenPlot
                        } else {
                            *t
                        }
                    })
                    .collect()
            })
            .collect();
        for y in 0..last_record.map.get_rows() {
            for x in 0..last_record.map.get_columns() {
                let point = Point::new(x as Int, y as Int);
                let terrain = last_record.map.point_get(&point).unwrap();
                match *terrain {
                    Terrain::Start | Terrain::PossibleLocation => {
                        for direction in vec![North, East, South, West] {
                            let neighbour_point = point.move_to(&direction);
                            let neighbour_option = last_record.map.point_get(&neighbour_point);
                            if let Some(neighbour) = neighbour_option {
                                match neighbour {
                                    Terrain::Start
                                    | Terrain::GardenPlot
                                    | Terrain::PossibleLocation => {
                                        *next_record.map.point_get_mut(&neighbour_point).unwrap() =
                                            Terrain::PossibleLocation;
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        self.map_history.push(next_record);
    }

    fn step(&mut self, times: Int) {
        for _ in 0..times {
            self.step_once();
        }
    }

    fn count_possible_locations(&self) -> Int {
        self.map_history
            .last()
            .unwrap()
            .map
            .iter()
            .flatten()
            .filter(|t| **t == Terrain::PossibleLocation)
            .count() as Int
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
        let mut garden = Garden::parse(input);
        let steps = if is_example { 6 } else { 64 };
        garden.step(steps);
        debug!(is_example, garden);
        garden.count_possible_locations()
    }

    fn solve_part_two(input: Input, is_example: bool) -> Answer {
        todo!()
    }
}

run!(DayTwentyOne);
