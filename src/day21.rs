mod grid;
mod problem;
use grid::*;
use problem::*;

type GardenMap = Map<Terrain>;
type Visited = HashMap<Point, Distance>;
type Distance = Int;

define_convertable_enum! {
    Terrain {
        Start => 'S',
        GardenPlot => '.',
        Rock => '#',
        PossibleLocation => 'O',
    }
}

#[derive(Debug)]
struct Garden {
    map: GardenMap,
    visited: Visited,
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
            visited: Visited::new(),
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
    fn run_breadth_first_search(mut self) -> Garden {
        let mut queue: Queue<Point> = Queue::new();
        let root = *self.map.find(|t| *t == Terrain::Start).first().unwrap();
        queue.push_back(root);
        self.visited.insert(root, 0);
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let distance = *self.visited.get(&current).unwrap();
            for node in self.map.get_adjacent(&current) {
                let terrain = self.map.point_get(&node).unwrap();
                if *terrain == Terrain::Start || *terrain == Terrain::GardenPlot {
                    if !self.visited.contains_key(&node) {
                        self.visited.insert(node, distance + 1);
                        queue.push_back(node);
                    }
                }
            }
        }
        self
    }

    fn count_possible_locations(&self, distance: Distance) -> Int {
        let parity = distance % 2;
        self.visited
            .iter()
            .filter(|(_, &d)| d % 2 == parity && d <= distance)
            .count() as Int
    }
}

struct DayTwentyOne {}

impl Problem for DayTwentyOne {
    const YEAR: Year = 2023;
    const DAY: Day = 21;
    const PART_ONE_EXPECTED: Answer = 3853;
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
        let garden = Garden::parse(input).run_breadth_first_search();
        let steps = if is_example { 6 } else { 64 };
        debug!(is_example, garden);
        garden.count_possible_locations(steps)
    }

    fn solve_part_two(input: Input, is_example: bool) -> Answer {
        let garden = Garden::parse(input);
        // First, using breadth-first search on the Garden we found all distances
        0
    }
}

run!(DayTwentyOne);
