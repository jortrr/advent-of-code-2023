use crate::grid::*;
use crate::*;

type GardenMap = Map<Terrain>;
type Visited = HashMap<Point, Distance>;
type Distance = Int;

define_convertable_enum! {
    Terrain {
        Start => 'S',
        GardenPlot => '.',
        Rock => '#',
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

pub struct DayTwentyOne {}

impl Problem for DayTwentyOne {
    fn year(&self) -> Year {
        2023
    }
    fn day(&self) -> Day {
        21
    }
    fn expect_part_one(&self) -> Answer {
        3853
    }
    fn expect_part_two(&self) -> Answer {
        639051580070841
    }

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

    fn solve_part_one(&self, input: Input, is_example: bool) -> Answer {
        let garden = Garden::parse(input).run_breadth_first_search();
        let steps = if is_example { 6 } else { 64 };
        debug!(is_example, garden);
        garden.count_possible_locations(steps)
    }

    fn solve_part_two(&self, input: Input, _is_example: bool) -> Answer {
        let garden = Garden::parse(input).run_breadth_first_search();
        // First, using breadth-first search on the Garden we found all distances
        // Then, we use the explanation given at https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21 to solve (not mine)
        let visited = garden.visited;
        let even_corners = visited
            .values()
            .filter(|v| **v % 2 == 0 && **v > 65)
            .count();
        let odd_corners = visited
            .values()
            .filter(|v| **v % 2 == 1 && **v > 65)
            .count();

        let even_full = visited.values().filter(|v| **v % 2 == 0).count();
        let odd_full = visited.values().filter(|v| **v % 2 == 1).count();

        let n = ((26501365 - (garden.map.get_rows() / 2)) / garden.map.get_rows()) as usize;
        assert_eq!(n, 202300);

        let solution = (n + 1).pow(2) * odd_full + n.pow(2) * even_full - (n + 1) * odd_corners
            + n * even_corners;
        solution as Answer
    }
}
