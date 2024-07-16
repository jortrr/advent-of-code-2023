mod grid;
mod macros;

use grid::*;

type Beam = Direction;

#[derive(Debug)]
struct Terrain {
    the_type: TerrainType,
    beams: Vec<Beam>,
    energized: bool,
}

impl Terrain {
    fn from_char(c: char) -> Terrain {
        Terrain {
            the_type: TerrainType::from_char(c),
            beams: Vec::new(),
            energized: false,
        }
    }
}

define_convertable_enum! {
    TerrainType {
        EmptySpace => '.',
        NorthEastMirror => '/',
        NorthWestMirror => '\\',
        NorthSouthSplitter => '|',
        EastWestSplitter => '-',
    }
}

impl TerrainType {
    fn redirect(&self, beam: &Beam) -> Vec<Beam> {
        use TerrainType::*;
        match (&self, beam) {
            (NorthEastMirror, North) => vec![East],
            (NorthEastMirror, East) => vec![North],
            (NorthEastMirror, South) => vec![West],
            (NorthEastMirror, West) => vec![South],
            (NorthWestMirror, North) => vec![West],
            (NorthWestMirror, East) => vec![South],
            (NorthWestMirror, South) => vec![East],
            (NorthWestMirror, West) => vec![North],
            (NorthSouthSplitter, East | West) => vec![North, South],
            (EastWestSplitter, North | South) => vec![East, West],
            (_, _) => vec![beam.clone()],
        }
    }
}

#[derive(Debug)]
struct ContraptionMap {
    rows: Int,
    columns: Int,
    grid: Grid<Terrain>,
}

impl ContraptionMap {
    fn from_strings(input: &Vec<String>) -> ContraptionMap {
        let grid: Grid<Terrain> = input
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().map(|c| Terrain::from_char(c)).collect::<Vec<_>>())
            .collect();
        let rows = grid.len() as Int;
        let columns = grid.first().unwrap().len() as Int;
        ContraptionMap {
            rows,
            columns,
            grid,
        }
    }

    fn from_string_slices(input: &Vec<&str>) -> ContraptionMap {
        let input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
        ContraptionMap::from_strings(&input)
    }

    fn get_terrain(&self, point: &Point) -> &Terrain {
        &self.grid[point.y as usize][point.x as usize]
    }

    fn get_terrain_mut(&mut self, point: &Point) -> &mut Terrain {
        &mut self.grid[point.y as usize][point.x as usize]
    }

    fn get_energy_map(&self) -> String {
        self.grid
            .iter()
            .map(|v| {
                v.iter()
                    .map(|t| if t.energized { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn get_terrain_map(&self) -> String {
        self.grid
            .iter()
            .map(|v| v.iter().map(|t| t.the_type.to_char()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn within_grid(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.columns && point.y >= 0 && point.y < self.rows
    }

    fn shoot_beam(&mut self, from: &Point, beam: Beam) {
        debug!(false, "shoot({:?}, {:?})", from, beam);
        if self.within_grid(from) {
            let current_terrain: &Terrain = self.get_terrain(from);
            if !current_terrain.beams.contains(&beam) {
                let current_terrain = self.get_terrain_mut(from);
                current_terrain.beams.push(beam.clone());
                current_terrain.energized = true;
                let redirected_beams = current_terrain.the_type.redirect(&beam);
                for redirected_beam in redirected_beams {
                    let next: Point = from.move_to(&redirected_beam);
                    self.shoot_beam(&next, redirected_beam);
                }
            }
        }
    }

    fn get_amount_of_energized_tiles(&self) -> Int {
        self.grid.iter().flatten().filter(|t| t.energized).count() as Int
    }
}

fn main() {
    println!("Hello, World! from src/day16.rs!");

    // Part 1 - Example
    let example_input = vec![
        r#".|...\...."#,
        r#"|.-.\....."#,
        r#".....|-..."#,
        r#"........|."#,
        r#".........."#,
        r#".........\"#,
        r#"..../.\\.."#,
        r#".-.-/..|.."#,
        r#".|....-|.\"#,
        r#"..//.|...."#,
    ];
    let mut example_map = ContraptionMap::from_string_slices(&example_input);
    example_map.shoot_beam(&Point::new(0, 0), East);
    // dbg!(&example_map);
    let terrain_map = example_map.get_terrain_map();
    let energy_map = example_map.get_energy_map();
    println!("Terrain map:\n{}\n", terrain_map);
    println!("Energy map:\n{}\n", energy_map);
    let example_expected_energized_map = vec![
        "######....",
        ".#...#....",
        ".#...#####",
        ".#...##...",
        ".#...##...",
        ".#...##...",
        ".#..####..",
        "########..",
        ".#######..",
        ".#...#.#..",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
    .join("\n");
    test!(example_expected_energized_map, energy_map);
    let amount_of_energized_tiles = example_map.get_amount_of_energized_tiles();
    test!(46, amount_of_energized_tiles);

    // Part 1
    let mut map = ContraptionMap::from_strings(&aoc_input::get(2023, 16));
    map.shoot_beam(&Point::new(0, 0), East);
    let amount_of_energized_tiles = map.get_amount_of_energized_tiles();
    test!(6906, amount_of_energized_tiles);
}
